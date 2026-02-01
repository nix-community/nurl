mod cli;
mod config;
mod fetcher;
mod prefetch;
mod revless;
mod simple;

use std::{
    fmt::{self, Display, Formatter},
    io::{IsTerminal, Write, stdout},
    str::Split,
};

use bstr::ByteSlice;
use clap::{Parser, ValueEnum};
use eyre::{Result, bail};
use gix_url::Scheme;
use supports_color::Stream;

use crate::{
    cli::{FetcherFunction, Opts},
    config::FetcherConfig,
    fetcher::{
        BuiltinsFetchGit, FetchCrate, FetchFromBitbucket, FetchFromGitHub, FetchFromGitLab,
        FetchFromGitea, FetchFromGitiles, FetchFromRepoOrCz, FetchFromSourcehut, FetchHex,
        FetchPypi, Fetcher, FetcherDispatch, Fetchgit, Fetchhg, Fetchpatch, Fetchpatch2, Fetchsvn,
        Fetchurl, Fetchzip,
    },
    prefetch::fod_prefetch,
};

pub struct Url<'a> {
    url: &'a str,
    path: &'a str,
}

impl Url<'_> {
    fn as_str(&self) -> &str {
        self.url
    }

    fn path_segments(&self) -> Split<'_, char> {
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
    if supports_color::on(Stream::Stderr).is_some() {
        color_eyre::install()?;
    }

    let opts = Opts::parse();
    let out = &mut stdout().lock();

    if let Some(expr) = opts.expr {
        write!(
            out,
            "{}",
            fod_prefetch(format!(
                r#"({expr}).overrideAttrs(_:{{outputHash="";outputHashAlgo="sha256";}})"#,
            ))?
        )?;

        if out.is_terminal() {
            writeln!(out)?;
        }

        return Ok(());
    }

    if opts.list_fetchers || opts.list_possible_fetchers {
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

    let url: gix_url::Url = opts.url.as_str().try_into()?;
    let path = url.path.to_str()?;
    let path = path.strip_prefix('/').unwrap_or(path);
    let path = path.split_once('?').map_or(path, |(path, _)| path);

    let fetcher: FetcherDispatch = match (opts.fetcher, url.host(), &url.scheme) {
        // high priority

        // prefer fetchpatch over fetchpatch2: https://github.com/NixOS/nixpkgs/issues/257446
        (None, ..) if path.ends_with(".diff") || path.ends_with(".patch") => Fetchpatch.into(),

        (None, ..) if is_archive(path) => Fetchzip.into(),

        // low priority
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
            Some(
                host @ ("codeberg.org" | "git.lix.systems" | "gitea.com" | "notabug.org"
                | "repo.palemoon.org"),
            ),
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

        (Some(FetcherFunction::Fetchpatch), ..) => Fetchpatch.into(),

        (Some(FetcherFunction::Fetchpatch2), ..) => Fetchpatch2.into(),

        (None, _, Scheme::Ext(scheme)) if scheme == "svn" => Fetchsvn.into(),
        (Some(FetcherFunction::Fetchsvn), ..) => Fetchsvn.into(),

        (Some(FetcherFunction::Fetchurl), ..) => Fetchurl.into(),

        (Some(FetcherFunction::Fetchzip), ..) => Fetchzip.into(),

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
            FetcherFunction::Fetchpatch => Fetchpatch.into(),
            FetcherFunction::Fetchpatch2 => Fetchpatch2.into(),
            FetcherFunction::Fetchsvn => Fetchsvn.into(),
            FetcherFunction::Fetchurl => Fetchurl.into(),
            FetcherFunction::Fetchzip => Fetchzip.into(),
        },
    };

    let url_bstring = url.to_bstring();
    let url = Url {
        url: url_bstring.to_str()?,
        path,
    };

    if opts.hash {
        fetcher.fetch_hash(out, &url, opts.into())?;
    } else if opts.json {
        fetcher.fetch_json(out, &url, opts.into())?;
    } else if opts.parse {
        fetcher.to_json(out, &url, opts.rev)?;
    } else {
        let mut cfg = FetcherConfig::from(opts);
        cfg.merge_overwrites();
        fetcher.fetch_nix(out, &url, cfg)?;
    }

    if out.is_terminal() {
        writeln!(out)?;
    }

    Ok(())
}

fn is_archive(path: &str) -> bool {
    let mut exts = path.rsplit('.');
    match exts.next() {
        Some("tar" | "tbz" | "tbz2" | "tgz" | "txz" | "zip") => true,
        Some(_) if exts.next() == Some("tar") => true,
        _ => false,
    }
}
