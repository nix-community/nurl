mod cli;
mod fetcher;

use anyhow::{bail, Result};
use clap::Parser;
use url::Host;

use crate::{
    cli::{Fetcher, Opts},
    fetcher::{
        FetchFromGitHub, FetchFromGitLab, FetchFromSourcehut, Fetchgit, Fetchhg,
        SimpleFlakeFetcher, UrlFlakeFetcher,
    },
};

use std::io::stdout;

fn main() -> Result<()> {
    let opts = Opts::parse();

    let out = &mut stdout().lock();
    let indent = &" ".repeat(opts.indent);

    match (opts.fetcher, opts.url.host()) {
        (Some(Fetcher::FetchFromGitHub), Some(host)) => {
            FetchFromGitHub((host != Host::Domain("github.com")).then_some(&host.to_string()))
                .fetch_nix(out, &opts.url, opts.rev, indent)?;
        }
        (None, Some(Host::Domain("github.com"))) => {
            FetchFromGitHub(None).fetch_nix(out, &opts.url, opts.rev, indent)?;
        }

        (Some(Fetcher::FetchFromGitLab), Some(host)) => {
            FetchFromGitLab((host != Host::Domain("github.com")).then_some(&host.to_string()))
                .fetch_nix(out, &opts.url, opts.rev, indent)?;
        }
        (None, Some(Host::Domain(host))) if host.starts_with("gitlab.") => {
            FetchFromGitLab((host != "gitlab.com").then_some(host))
                .fetch_nix(out, &opts.url, opts.rev, indent)?;
        }

        (Some(Fetcher::FetchFromSourcehut), Some(host)) => {
            FetchFromSourcehut((host != Host::Domain("git.sr.ht")).then_some(&host.to_string()))
                .fetch_nix(out, &opts.url, opts.rev, indent)?;
        }
        (None, Some(Host::Domain("git.sr.ht"))) => {
            FetchFromSourcehut(None).fetch_nix(out, &opts.url, opts.rev, indent)?;
        }

        (
            Some(Fetcher::FetchFromGitHub | Fetcher::FetchFromGitLab | Fetcher::FetchFromSourcehut),
            None,
        ) => {
            bail!("bad");
        }

        (Some(Fetcher::Fetchgit), _) | (None, _) => {
            Fetchgit.fetch_nix(out, &opts.url, opts.rev, indent)?;
        }

        (Some(Fetcher::Fetchhg), _) => {
            Fetchhg.fetch_nix(out, &opts.url, opts.rev, indent)?;
        }
    }

    Ok(())
}
