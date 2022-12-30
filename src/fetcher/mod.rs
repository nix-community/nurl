mod git;
mod github;
mod gitlab;
mod hg;
mod sourcehut;

use enum_dispatch::enum_dispatch;
pub use git::Fetchgit;
pub use github::FetchFromGitHub;
pub use gitlab::FetchFromGitLab;
pub use hg::Fetchhg;
use indoc::writedoc;
use serde_json::json;
pub use sourcehut::FetchFromSourcehut;

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use url::Url;

use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

#[enum_dispatch]
pub trait Fetcher {
    fn fetch_nix(&self, out: &mut impl Write, url: Url, rev: String, indent: String) -> Result<()>;
    fn fetch_json(&self, out: &mut impl Write, url: Url, rev: String) -> Result<()>;
}

#[enum_dispatch(Fetcher)]
pub enum FetcherDispatch {
    FetchFromGitHub(FetchFromGitHub),
    FetchFromGitLab(FetchFromGitLab),
    FetchFromSourcehut(FetchFromSourcehut),
    Fetchgit(Fetchgit),
    Fetchhg(Fetchhg),
}

#[macro_export]
macro_rules! impl_fetcher {
    ($t:ident $($tt:tt)*) => {
        impl $($tt)* $crate::fetcher::Fetcher for $t $($tt)* {
            fn fetch_nix(
                &self,
                out: &mut impl ::std::io::Write,
                url: ::url::Url,
                rev: String,
                indent: String,
            ) -> ::anyhow::Result<()> {
                self.fetch_nix_imp(out, url, rev, indent)
            }

            fn fetch_json(&self, out: &mut impl ::std::io::Write, url: ::url::Url, rev: String) -> ::anyhow::Result<()> {
                self.fetch_json_imp(out, url, rev)
            }
        }
    };
}

pub trait SimpleFlakeFetcher<'a> {
    const FLAKE_TYPE: &'static str;
    const NAME: &'static str;

    fn host(&'a self) -> &'a Option<String>;

    fn get_repo(&self, url: &Url) -> Option<(String, String)> {
        let mut xs = url.path_segments()?;
        let owner = xs.next()?;
        let repo = xs.next()?;
        Some((
            owner.into(),
            repo.strip_suffix(".git").unwrap_or(repo).into(),
        ))
    }

    fn fetch(&'a self, url: &Url, rev: &str) -> Result<(String, String, String)> {
        let (owner, repo) = self
            .get_repo(url)
            .with_context(|| format!("failed to parse {url} as a {} url", Self::FLAKE_TYPE))?;

        let hash = flake_prefetch(if let Some(host) = self.host() {
            format!("{}:{owner}/{repo}/{rev}?host={}", Self::FLAKE_TYPE, host)
        } else {
            format!("{}:{owner}/{repo}/{rev}", Self::FLAKE_TYPE)
        })?;

        Ok((owner, repo, hash))
    }

    fn fetch_nix_imp(
        &'a self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        indent: String,
    ) -> Result<()> {
        let (owner, repo, hash) = self.fetch(&url, &rev)?;

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

    fn fetch_json_imp(&'a self, out: &mut impl Write, url: Url, rev: String) -> Result<()> {
        let (owner, repo, hash) = self.fetch(&url, &rev)?;

        let mut args = json! ({
            "owner": owner,
            "repo": repo,
            "rev": rev,
            "hash": hash,
        });

        if let Some(host) = self.host() {
            args["host"] = json!(host);
        }

        serde_json::to_writer(
            out,
            &json!({
                "fetcher": Self::NAME,
                "args": args,
            }),
        )?;

        Ok(())
    }
}

pub trait UrlFlakeFetcher {
    const FLAKE_TYPE: &'static str;
    const NAME: &'static str;

    fn fetch(&self, url: &Url, rev: &str) -> Result<String> {
        flake_prefetch(format!(
            "{}+{url}?{}={rev}",
            Self::FLAKE_TYPE,
            if rev.len() == 40 { "rev" } else { "ref" },
        ))
    }

    fn fetch_nix_imp(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        indent: String,
    ) -> Result<()> {
        let hash = self.fetch(&url, &rev)?;

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

    fn fetch_json_imp(&self, out: &mut impl Write, url: Url, rev: String) -> Result<()> {
        let hash = self.fetch(&url, &rev)?;

        serde_json::to_writer(
            out,
            &json!({
                "fetcher": Self::NAME,
                "args": {
                    "url": url.to_string(),
                    "rev": rev,
                    "hash": hash,
                },
            }),
        )?;

        Ok(())
    }
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
