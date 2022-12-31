#![allow(clippy::too_many_arguments)]

mod cli;
mod common;
mod fetcher;

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use itertools::Itertools;
use url::Host;

use crate::{
    cli::{FetcherFunction, Opts},
    fetcher::{
        FetchFromBitBucket, FetchFromGitHub, FetchFromGitLab, FetchFromGitea, FetchFromRepoOrCz,
        FetchFromSourcehut, Fetcher, FetcherDispatch, Fetchgit, Fetchhg,
    },
};

use std::io::{stdout, Write};

fn main() -> Result<()> {
    let opts = Opts::parse();

    if opts.list_fetchers || opts.list_possible_fetchers {
        let mut out = stdout().lock();
        for fetcher in FetcherFunction::value_variants() {
            if matches!(fetcher, FetcherFunction::Fetchhg) && opts.list_possible_fetchers {
                continue;
            }
            if let Some(fetcher) = fetcher.to_possible_value() {
                writeln!(out, "{}", fetcher.get_name())?;
            }
        }
        return Ok(());
    }

    let fetcher: FetcherDispatch = match (opts.fetcher, opts.url.host()) {
        (None | Some(FetcherFunction::FetchFromBitBucket), Some(Host::Domain("bitbucket.org"))) => {
            FetchFromBitBucket.into()
        }
        (Some(FetcherFunction::FetchFromBitBucket), _) => {
            bail!("fetchFromBitBucket only supports bitbucket.org");
        }

        (None | Some(FetcherFunction::FetchFromGitHub), Some(Host::Domain("github.com"))) => {
            FetchFromGitHub(None).into()
        }
        (Some(FetcherFunction::FetchFromGitHub), Some(host)) => {
            FetchFromGitHub(Some(host.to_string())).into()
        }

        (None | Some(FetcherFunction::FetchFromGitLab), Some(Host::Domain("gitlab.com"))) => {
            FetchFromGitLab(None).into()
        }
        (None, Some(Host::Domain(host))) if host.starts_with("gitlab.") => {
            FetchFromGitLab(Some(host.into())).into()
        }
        (Some(FetcherFunction::FetchFromGitLab), Some(host)) => {
            FetchFromGitLab(Some(host.to_string())).into()
        }

        (
            None | Some(FetcherFunction::FetchFromGitea),
            Some(Host::Domain(host @ "codeberg.org")),
        ) => FetchFromGitea(host.into()).into(),
        (Some(FetcherFunction::FetchFromGitea), Some(host)) => {
            FetchFromGitea(host.to_string()).into()
        }

        (None | Some(FetcherFunction::FetchFromRepoOrCz), Some(Host::Domain("repo.or.cz"))) => {
            FetchFromRepoOrCz.into()
        }
        (Some(FetcherFunction::FetchFromRepoOrCz), _) => {
            bail!("fetchFromRepoOrCz only supports repo.or.cz");
        }

        (None | Some(FetcherFunction::FetchFromSourcehut), Some(Host::Domain("git.sr.ht"))) => {
            FetchFromSourcehut(None).into()
        }
        (Some(FetcherFunction::FetchFromSourcehut), Some(host)) => {
            FetchFromSourcehut(Some(host.to_string())).into()
        }

        (
            Some(
                fetcher @ (FetcherFunction::FetchFromGitHub
                | FetcherFunction::FetchFromGitLab
                | FetcherFunction::FetchFromGitea
                | FetcherFunction::FetchFromSourcehut),
            ),
            None,
        ) => {
            bail!("{fetcher:?} does not support URLs without a host");
        }

        (None | Some(FetcherFunction::Fetchgit), _) => Fetchgit.into(),

        (Some(FetcherFunction::Fetchhg), _) => Fetchhg.into(),
    };

    let out = &mut stdout().lock();
    let args = opts.args.into_iter().tuples().collect();
    if opts.json {
        fetcher.fetch_json(out, &opts.url, opts.rev, args)
    } else {
        fetcher.fetch_nix(out, &opts.url, opts.rev, args, " ".repeat(opts.indent))
    }?;

    Ok(())
}
