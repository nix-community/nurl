#![allow(clippy::too_many_arguments)]

mod cli;
mod fetcher;
mod prefetch;
mod simple;

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::{
    cli::{FetcherFunction, Opts},
    fetcher::{
        FetchFromBitbucket, FetchFromGitHub, FetchFromGitLab, FetchFromGitea, FetchFromGitiles,
        FetchFromRepoOrCz, FetchFromSourcehut, Fetcher, FetcherDispatch, Fetchgit, Fetchhg,
        Fetchsvn,
    },
};

use std::io::{stdout, Write};

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

    let fetcher: FetcherDispatch = match (opts.fetcher, opts.url.host_str(), opts.url.scheme()) {
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
        (Some(FetcherFunction::FetchFromGitLab), Some(host), _) => {
            FetchFromGitLab::new(Some(host)).into()
        }

        (
            None | Some(FetcherFunction::FetchFromGitea),
            Some(host @ ("codeberg.org" | "gitea.com" | "notabug.org")),
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

        (None | Some(FetcherFunction::Fetchgit), _, "git") => Fetchgit(GitScheme::Yes).into(),
        (None | Some(FetcherFunction::Fetchgit), _, scheme) if scheme.starts_with("git+") => {
            Fetchgit(GitScheme::Plus).into()
        }
        (Some(FetcherFunction::Fetchgit), ..) => Fetchgit(GitScheme::No).into(),

        (None | Some(FetcherFunction::Fetchhg), _, scheme) if scheme.starts_with("hg+") => {
            Fetchhg(true).into()
        }
        (Some(FetcherFunction::Fetchhg), ..) => Fetchhg(false).into(),

        (None, _, "svn") => Fetchsvn.into(),
        (Some(FetcherFunction::Fetchsvn), ..) => Fetchsvn.into(),

        (None, ..) => Fetchgit(GitScheme::No).into(),
    };

    let out = &mut stdout().lock();
    let args = opts.args.into_iter().tuples().collect();
    let args_str = opts.args_str.into_iter().tuples().collect();
    if opts.json {
        fetcher.fetch_json(
            out,
            &opts.url,
            opts.rev,
            args,
            args_str,
            opts.overwrites.into_iter().tuples().collect(),
            opts.overwrites_str.into_iter().tuples().collect(),
        )
    } else {
        let mut overwrites: FxHashMap<_, _> = opts.overwrites.into_iter().tuples().collect();

        for (key, value) in opts.overwrites_str.into_iter().tuples() {
            overwrites.insert(key, format!(r#""{value}""#));
        }

        fetcher.fetch_nix(
            out,
            &opts.url,
            opts.rev,
            args,
            args_str,
            overwrites,
            " ".repeat(opts.indent),
        )
    }?;

    Ok(())
}
