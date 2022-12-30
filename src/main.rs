mod cli;
mod fetcher;

use anyhow::{bail, Result};
use clap::Parser;
use url::Host;

use crate::{
    cli::{FetcherFunction, Opts},
    fetcher::{
        FetchFromGitHub, FetchFromGitLab, FetchFromSourcehut, Fetcher, FetcherDispatch, Fetchgit,
        Fetchhg,
    },
};

use std::io::stdout;

fn main() -> Result<()> {
    let opts = Opts::parse();

    let indent = &" ".repeat(opts.indent);

    let fetcher: FetcherDispatch = match (opts.fetcher, opts.url.host()) {
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
                | FetcherFunction::FetchFromSourcehut),
            ),
            None,
        ) => {
            bail!("{fetcher:?} does not support URLs without a host");
        }

        (Some(FetcherFunction::Fetchgit), _) | (None, _) => Fetchgit.into(),

        (Some(FetcherFunction::Fetchhg), _) => Fetchhg.into(),
    };

    fetcher.fetch_nix(&mut stdout().lock(), &opts.url, opts.rev, indent)?;

    Ok(())
}
