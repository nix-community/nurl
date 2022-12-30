mod git;
mod github;
mod gitlab;
mod hg;
mod sourcehut;

pub use git::Fetchgit;
pub use github::FetchFromGitHub;
pub use gitlab::FetchFromGitLab;
pub use hg::Fetchhg;
use indoc::writedoc;
pub use sourcehut::FetchFromSourcehut;

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use url::Url;

use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

pub trait Fetcher {
    fn fetch_nix(&self, out: &mut impl Write, url: &Url, rev: String, indent: &str) -> Result<()>;
}

trait GetStdout {
    fn get_stdout(&mut self) -> Result<Vec<u8>>;
}

impl GetStdout for Command {
    fn get_stdout(&mut self) -> Result<Vec<u8>> {
        let Output { stdout, status, .. } = self.stderr(Stdio::inherit()).output()?;
        if !status.success() {
            bail!("command exited with exit code {}", status);
        }
        Ok(stdout)
    }
}

pub fn flake_prefetch(flake_ref: String) -> Result<String> {
    #[derive(Deserialize)]
    struct PrefetchOutput {
        hash: String,
    }

    eprintln!("$ nix flake prefetch --json {flake_ref}");
    Ok(serde_json::from_slice::<PrefetchOutput>(
        &Command::new("nix")
            .arg("flake")
            .arg("prefetch")
            .arg("--json")
            .arg(flake_ref)
            .get_stdout()?,
    )?
    .hash)
}

pub trait SimpleFlakeFetcher<'a> {
    const NAME: &'static str;
    const FLAKE_TYPE: &'static str;

    fn host(&self) -> Option<&'a str>;

    fn get_repo(&self, url: &Url) -> Option<(String, String)> {
        let mut xs = url.path_segments()?;
        let owner = xs.next()?;
        let repo = xs.next()?;
        Some((
            owner.into(),
            repo.strip_suffix(".git").unwrap_or(repo).into(),
        ))
    }

    fn fetch_nix(&self, out: &mut impl Write, url: &Url, rev: String, indent: &str) -> Result<()> {
        let (owner, repo) = self
            .get_repo(url)
            .with_context(|| format!("failed to parse {url} as a {} url", Self::FLAKE_TYPE))?;

        let hash = flake_prefetch(if let Some(host) = self.host() {
            format!("{}:{owner}/{repo}/{rev}?host={}", Self::FLAKE_TYPE, host)
        } else {
            format!("{}:{owner}/{repo}/{rev}", Self::FLAKE_TYPE)
        })?;

        writeln!(out, "{} {{", Self::NAME)?;

        if let Some(domain) = self.host() {
            writeln!(out, r#"{indent}  domain = "{domain}";"#)?;
        }

        writedoc!(
            out,
            r#"
                {indent}  owner = "{owner}";
                {indent}  repo = "{repo}";
                {indent}  rev = "{rev}";
                {indent}  hash = "{hash}";
                {indent}}}"#,
        )?;

        Ok(())
    }
}

pub(crate) trait UrlFlakeFetcher {
    const NAME: &'static str;
    const FLAKE_TYPE: &'static str;

    fn fetch_nix(&self, out: &mut impl Write, url: &Url, rev: String, indent: &str) -> Result<()> {
        let hash = flake_prefetch(format!(
            "{}+{url}?{}={rev}",
            Self::FLAKE_TYPE,
            if rev.len() == 40 { "rev" } else { "ref" },
        ))?;

        writedoc!(
            out,
            r#"
                {} {{
                {indent}  url = "{url}";
                {indent}  rev = "{rev}";
                {indent}  hash = "{hash}";
                {indent}}}"#,
            Self::NAME,
        )?;

        Ok(())
    }
}
