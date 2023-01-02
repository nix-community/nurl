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

fn main() -> Result<()> {
    let opts = Opts::parse();

    if opts.list_fetchers || opts.list_possible_fetchers {
        let mut out = stdout().lock();
        for fetcher in FetcherFunction::value_variants() {
            if opts.list_possible_fetchers
                && matches!(
                    fetcher,
                    FetcherFunction::Fetchhg | FetcherFunction::Fetchsvn
                )
            {
                continue;
            }
            if let Some(fetcher) = fetcher.to_possible_value() {
                writeln!(out, "{}", fetcher.get_name())?;
            }
        }
        return Ok(());
    }

    let fetcher: FetcherDispatch = match (opts.fetcher, opts.url.host_str()) {
        (None | Some(FetcherFunction::FetchFromBitbucket), Some("bitbucket.org")) => {
            FetchFromBitbucket.into()
        }
        (Some(FetcherFunction::FetchFromBitbucket), _) => {
            bail!("fetchFromBitbucket only supports bitbucket.org");
        }

        (None | Some(FetcherFunction::FetchFromGitHub), Some("github.com")) => {
            FetchFromGitHub(None).into()
        }
        (Some(FetcherFunction::FetchFromGitHub), Some(host)) => FetchFromGitHub(Some(host)).into(),

        (None | Some(FetcherFunction::FetchFromGitLab), Some("gitlab.com")) => {
            FetchFromGitLab::new(None).into()
        }
        (None, Some(host)) if host.starts_with("gitlab.") => {
            FetchFromGitLab::new(Some(host)).into()
        }
        (Some(FetcherFunction::FetchFromGitLab), Some(host)) => {
            FetchFromGitLab::new(Some(host)).into()
        }

        (None | Some(FetcherFunction::FetchFromGitea), Some(host @ "codeberg.org")) => {
            FetchFromGitea(host).into()
        }
        (Some(FetcherFunction::FetchFromGitea), Some(host)) => FetchFromGitea(host).into(),

        (None | Some(FetcherFunction::FetchFromGitiles), Some(host))
            if host.ends_with(".googlesource.com") =>
        {
            FetchFromGitiles.into()
        }
        (Some(FetcherFunction::FetchFromGitiles), _) => FetchFromGitiles.into(),

        (None | Some(FetcherFunction::FetchFromRepoOrCz), Some("repo.or.cz")) => {
            FetchFromRepoOrCz.into()
        }
        (Some(FetcherFunction::FetchFromRepoOrCz), _) => {
            bail!("fetchFromRepoOrCz only supports repo.or.cz");
        }

        (None | Some(FetcherFunction::FetchFromSourcehut), Some("git.sr.ht")) => {
            FetchFromSourcehut(None).into()
        }
        (Some(FetcherFunction::FetchFromSourcehut), Some(host)) => {
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
        ) => {
            bail!("{fetcher:?} does not support URLs without a host");
        }

        (None | Some(FetcherFunction::Fetchgit), _) => Fetchgit.into(),

        (Some(FetcherFunction::Fetchhg), _) => Fetchhg.into(),

        (Some(FetcherFunction::Fetchsvn), _) => Fetchsvn.into(),
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
