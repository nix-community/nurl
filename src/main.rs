#![allow(clippy::too_many_arguments)]

mod cli;
mod fetcher;
mod prefetch;
mod simple;

use anyhow::{bail, Result};
use bstr::ByteSlice;
use clap::{Parser, ValueEnum};
use gix_url::Scheme;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::{
    cli::{FetcherFunction, Opts},
    fetcher::{
        BuiltinsFetchGit, FetchCrate, FetchFromBitbucket, FetchFromGitHub, FetchFromGitLab,
        FetchFromGitea, FetchFromGitiles, FetchFromRepoOrCz, FetchFromSourcehut, FetchHex,
        FetchPypi, Fetcher, FetcherDispatch, Fetchgit, Fetchhg, Fetchsvn,
    },
};

use std::{
    fmt::{self, Display, Formatter},
    io::{stdout, Write},
    str::Split,
};

pub struct Url<'a> {
    url: &'a str,
    path: &'a str,
}

impl Url<'_> {
    fn as_str(&self) -> &str {
        self.url
    }

    fn path_segments(&self) -> Split<char> {
        self.path.split('/')
    }
}

impl Display for Url<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub enum GitScheme {
    Yes,
    No,
    Plus,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if opts.list_fetchers || opts.list_possible_fetchers {
        let mut out = stdout().lock();
        let fetchers = FetcherFunction::value_variants()
            .iter()
            .filter(|fetcher| {
                opts.list_fetchers || !matches!(fetcher, FetcherFunction::BuiltinsFetchGit)
            })
            .filter_map(ValueEnum::to_possible_value);

        if let Some(sep) = opts.list_sep {
            let mut fetchers = fetchers;
            if let Some(fetcher) = fetchers.next() {
                write!(out, "{}", fetcher.get_name())?;
            }
            for fetcher in fetchers {
                write!(out, "{}{}", sep, fetcher.get_name())?;
            }
        } else {
            for fetcher in fetchers {
                writeln!(out, "{}", fetcher.get_name())?;
            }
        }

        return Ok(());
    }

    let url: gix_url::Url = opts.url.try_into()?;

    let fetcher: FetcherDispatch = match (opts.fetcher, url.host(), &url.scheme) {
        (Some(FetcherFunction::BuiltinsFetchGit), ..) => BuiltinsFetchGit.into(),

        (None | Some(FetcherFunction::FetchCrate), Some("crates.io"), _) => FetchCrate(true).into(),
        (None | Some(FetcherFunction::FetchCrate), Some("lib.rs"), _) => FetchCrate(false).into(),
        (Some(FetcherFunction::FetchCrate), ..) => {
            bail!("fetchCrate only supports crates.io and lib.rs");
        }

        (None | Some(FetcherFunction::FetchFromBitbucket), Some("bitbucket.org"), _) => {
            FetchFromBitbucket.into()
        }
        (Some(FetcherFunction::FetchFromBitbucket), ..) => {
            bail!("fetchFromBitbucket only supports bitbucket.org");
        }

        (None | Some(FetcherFunction::FetchFromGitHub), Some("github.com"), _) => {
            FetchFromGitHub(None).into()
        }
        (Some(FetcherFunction::FetchFromGitHub), Some(host), _) => {
            FetchFromGitHub(Some(host)).into()
        }

        (None | Some(FetcherFunction::FetchFromGitLab), Some("gitlab.com"), _) => {
            FetchFromGitLab::new(None).into()
        }
        (None, Some(host), _) if host.starts_with("gitlab.") => {
            FetchFromGitLab::new(Some(host)).into()
        }
        (None, Some(host @ ("invent.kde.org" | "salsa.debian.org")), _) => {
            FetchFromGitLab::new(Some(host)).into()
        }
        (Some(FetcherFunction::FetchFromGitLab), Some(host), _) => {
            FetchFromGitLab::new(Some(host)).into()
        }

        (
            None | Some(FetcherFunction::FetchFromGitea),
            Some(host @ ("codeberg.org" | "gitea.com" | "notabug.org" | "repo.palemoon.org")),
            _,
        ) => FetchFromGitea(host).into(),
        (Some(FetcherFunction::FetchFromGitea), Some(host), _) => FetchFromGitea(host).into(),

        (None | Some(FetcherFunction::FetchFromGitiles), Some(host), _)
            if host.ends_with(".googlesource.com") =>
        {
            FetchFromGitiles.into()
        }
        (Some(FetcherFunction::FetchFromGitiles), ..) => FetchFromGitiles.into(),

        (None | Some(FetcherFunction::FetchFromRepoOrCz), Some("repo.or.cz"), _) => {
            FetchFromRepoOrCz.into()
        }
        (Some(FetcherFunction::FetchFromRepoOrCz), ..) => {
            bail!("fetchFromRepoOrCz only supports repo.or.cz");
        }

        (None | Some(FetcherFunction::FetchFromSourcehut), Some("git.sr.ht"), _) => {
            FetchFromSourcehut(None).into()
        }
        (Some(FetcherFunction::FetchFromSourcehut), Some(host), _) => {
            FetchFromSourcehut(Some(host)).into()
        }

        (
            Some(
                fetcher @ (FetcherFunction::FetchFromGitHub
                | FetcherFunction::FetchFromGitLab
                | FetcherFunction::FetchFromGitea
                | FetcherFunction::FetchFromSourcehut),
            ),
            None,
            _,
        ) => {
            bail!("{fetcher:?} does not support URLs without a host");
        }

        (None | Some(FetcherFunction::FetchHex), Some("hex.pm"), _) => FetchHex.into(),
        (Some(FetcherFunction::FetchHex), ..) => {
            bail!("fetchHex only supports hex.pm");
        }

        (None | Some(FetcherFunction::FetchPypi), Some("pypi.org"), _) => FetchPypi.into(),
        (Some(FetcherFunction::FetchPypi), ..) => {
            bail!("fetchPypi only supports pypi.org");
        }

        (None | Some(FetcherFunction::Fetchgit), _, Scheme::Git) => Fetchgit(GitScheme::Yes).into(),
        (None | Some(FetcherFunction::Fetchgit), _, Scheme::Ext(scheme))
            if scheme.starts_with("git+") =>
        {
            Fetchgit(GitScheme::Plus).into()
        }
        (Some(FetcherFunction::Fetchgit), ..) => Fetchgit(GitScheme::No).into(),

        (None | Some(FetcherFunction::Fetchhg), _, Scheme::Ext(scheme))
            if scheme.starts_with("hg+") =>
        {
            Fetchhg(true).into()
        }
        (Some(FetcherFunction::Fetchhg), ..) => Fetchhg(false).into(),

        (None, _, Scheme::Ext(scheme)) if scheme == "svn" => Fetchsvn.into(),
        (Some(FetcherFunction::Fetchsvn), ..) => Fetchsvn.into(),

        (None, ..) => match opts.fallback {
            FetcherFunction::BuiltinsFetchGit => BuiltinsFetchGit.into(),
            FetcherFunction::FetchCrate => {
                bail!("fetchCrate only supports crates.io and lib.rs");
            }
            FetcherFunction::FetchFromBitbucket => {
                bail!("fetchFromBitbucket only supports bitbucket.org");
            }
            fetcher @ (FetcherFunction::FetchFromGitHub
            | FetcherFunction::FetchFromGitLab
            | FetcherFunction::FetchFromGitea
            | FetcherFunction::FetchFromSourcehut) => {
                bail!("{fetcher:?} does not support URLs without a host");
            }
            FetcherFunction::FetchFromGitiles => FetchFromGitiles.into(),
            FetcherFunction::FetchFromRepoOrCz => {
                bail!("fetchFromRepoOrCz only supports repo.or.cz");
            }
            FetcherFunction::FetchHex => {
                bail!("fetchHex only supports hex.pm");
            }
            FetcherFunction::FetchPypi => {
                bail!("fetchPypi only supports pypi.org");
            }
            FetcherFunction::Fetchgit => Fetchgit(GitScheme::No).into(),
            FetcherFunction::Fetchhg => Fetchhg(false).into(),
            FetcherFunction::Fetchsvn => Fetchsvn.into(),
        },
    };

    let url_bstring = url.to_bstring();
    let path = url.path.to_str()?;
    let url = Url {
        url: url_bstring.to_str()?,
        path: path.strip_prefix('/').unwrap_or(path),
    };

    let out = &mut stdout().lock();
    let args = opts.args.into_iter().tuples().collect();
    let args_str = opts.args_str.into_iter().tuples().collect();
    if opts.hash {
        fetcher.fetch_hash(
            out,
            &url,
            opts.rev,
            opts.submodules,
            args,
            args_str,
            opts.nixpkgs,
        )
    } else if opts.json {
        fetcher.fetch_json(
            out,
            &url,
            opts.rev,
            opts.submodules,
            args,
            args_str,
            opts.overwrites.into_iter().tuples().collect(),
            opts.overwrites_str.into_iter().tuples().collect(),
            opts.nixpkgs,
        )
    } else if opts.parse {
        fetcher.to_json(out, &url, opts.rev)
    } else {
        let mut overwrites: FxHashMap<_, _> = opts.overwrites.into_iter().tuples().collect();

        for (key, value) in opts.overwrites_str.into_iter().tuples() {
            overwrites.insert(key, format!(r#""{value}""#));
        }

        fetcher.fetch_nix(
            out,
            &url,
            opts.rev,
            opts.submodules,
            args,
            args_str,
            overwrites,
            opts.nixpkgs,
            " ".repeat(opts.indent),
        )
    }
}
