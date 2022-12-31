mod git;
mod github;
mod gitlab;
mod hg;
mod sourcehut;

pub use git::Fetchgit;
pub use github::FetchFromGitHub;
pub use gitlab::FetchFromGitLab;
pub use hg::Fetchhg;
pub use sourcehut::FetchFromSourcehut;

use enum_dispatch::enum_dispatch;
use indoc::writedoc;
use serde_json::json;

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use url::Url;

use std::{
    fmt::Write as _,
    io::{BufRead, Write},
    process::{Command, Output, Stdio},
};

#[enum_dispatch]
pub trait Fetcher {
    fn fetch_nix(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()>;
    fn fetch_json(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()>;
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
    ($t:ty) => {
        impl $crate::fetcher::Fetcher for $t {
            fn fetch_nix(
                &self,
                out: &mut impl ::std::io::Write,
                url: ::url::Url,
                rev: String,
                args: Vec<(String, String)>,
                indent: String,
            ) -> ::anyhow::Result<()> {
                self.fetch_nix_impl(out, url, rev, args, indent)
            }

            fn fetch_json(
                &self,
                out: &mut impl ::std::io::Write,
                url: ::url::Url,
                rev: String,
                args: Vec<(String, String)>,
            ) -> ::anyhow::Result<()> {
                self.fetch_json_impl(out, url, rev, args)
            }
        }
    };
}

pub trait SimpleFetcher<'a> {
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

    fn fetch_fod(
        &'a self,
        url: &Url,
        rev: &str,
        args: &[(String, String)],
    ) -> Result<(String, String, String)> {
        let (owner, repo) = self
            .get_repo(url)
            .with_context(|| format!("failed to parse {url}"))?;

        let mut expr = format!(
            r#"(import <nixpkgs> {{}}).{}{{owner="{owner}";repo="{repo}";rev="{rev}";hash="sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";"#,
            Self::NAME
        );
        if let Some(host) = self.host() {
            write!(expr, r#"domain="{host}""#)?;
        }
        for (key, value) in args {
            write!(expr, "{key}={value};")?;
        }
        expr.push('}');

        let hash = fod_prefetch(expr)?;

        Ok((owner, repo, hash))
    }

    fn write_nix(
        &'a self,
        out: &mut impl Write,
        owner: String,
        repo: String,
        rev: String,
        hash: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
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
            "#
        )?;

        for (key, value) in args {
            writeln!(out, "{indent}  {key} = {value};")?;
        }

        write!(out, "{indent}}}")?;

        Ok(())
    }

    fn write_json(
        &'a self,
        out: &mut impl Write,
        owner: String,
        repo: String,
        rev: String,
        hash: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let mut fetcher_args = json! ({
            "owner": owner,
            "repo": repo,
            "rev": rev,
            "hash": hash,
        });

        if let Some(host) = self.host() {
            fetcher_args["host"] = json!(host);
        }

        for (key, value) in args {
            fetcher_args[key] = json!(value);
        }

        serde_json::to_writer(
            out,
            &json!({
                "fetcher": Self::NAME,
                "args": fetcher_args,
            }),
        )?;

        Ok(())
    }
}

pub trait SimpleFodFetcher<'a>: SimpleFetcher<'a> {
    fn fetch_nix_impl(
        &'a self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
        let (owner, repo, hash) = self.fetch_fod(&url, &rev, &args)?;
        self.write_nix(out, owner, repo, rev, hash, args, indent)
    }

    fn fetch_json_impl(
        &'a self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let (owner, repo, hash) = self.fetch_fod(&url, &rev, &args)?;
        self.write_json(out, owner, repo, rev, hash, args)
    }
}

pub trait SimpleFlakeFetcher<'a>: SimpleFetcher<'a> {
    const FLAKE_TYPE: &'static str;

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

    fn fetch_nix_impl(
        &'a self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
        let (owner, repo, hash) = if args.is_empty() {
            self.fetch(&url, &rev)?
        } else {
            self.fetch_fod(&url, &rev, &args)?
        };

        self.write_nix(out, owner, repo, rev, hash, args, indent)
    }

    fn fetch_json_impl(
        &'a self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let (owner, repo, hash) = if args.is_empty() {
            self.fetch(&url, &rev)?
        } else {
            self.fetch_fod(&url, &rev, &args)?
        };
        self.write_json(out, owner, repo, rev, hash, args)
    }
}

pub trait UrlFetcher {
    const NAME: &'static str;

    fn fetch_fod(&self, url: &Url, rev: &str, args: &[(String, String)]) -> Result<String> {
        let mut expr = format!(
            r#"(import <nixpkgs> {{}}).{}{{url="{url}";rev="{rev}";hash="sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";"#,
            Self::NAME
        );
        for (key, value) in args {
            write!(expr, "{key}={value};")?;
        }
        expr.push('}');
        fod_prefetch(expr)
    }

    fn write_nix(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        hash: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
        writedoc!(
            out,
            r#"
                {} {{
                {indent}  url = "{url}";
                {indent}  rev = "{rev}";
                {indent}  hash = "{hash}";
            "#,
            Self::NAME
        )?;

        for (key, value) in args {
            writeln!(out, "{indent}  {key} = {value};")?;
        }

        write!(out, "{indent}}}")?;

        Ok(())
    }

    fn write_json(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        hash: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let mut fetcher_args = json!({
            "url": url.to_string(),
            "rev": rev,
            "hash": hash,
        });

        for (key, value) in args {
            fetcher_args[key] = json!(value);
        }

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

pub trait UrlFodFetcher: UrlFetcher {
    fn fetch_nix_impl(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
        let hash = self.fetch_fod(&url, &rev, &args)?;
        self.write_nix(out, url, rev, hash, args, indent)
    }

    fn fetch_json_impl(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let hash = self.fetch_fod(&url, &rev, &args)?;
        self.write_json(out, url, rev, hash, args)
    }
}

pub trait UrlFlakeFetcher: UrlFetcher {
    const FLAKE_TYPE: &'static str;

    fn fetch(&self, url: &Url, rev: &str) -> Result<String> {
        flake_prefetch(format!(
            "{}+{url}?{}={rev}",
            Self::FLAKE_TYPE,
            if rev.len() == 40 { "rev" } else { "ref" },
        ))
    }

    fn fetch_nix_impl(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
        let hash = if args.is_empty() {
            self.fetch(&url, &rev)?
        } else {
            self.fetch_fod(&url, &rev, &args)?
        };
        self.write_nix(out, url, rev, hash, args, indent)
    }

    fn fetch_json_impl(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let hash = if args.is_empty() {
            self.fetch(&url, &rev)?
        } else {
            self.fetch_fod(&url, &rev, &args)?
        };
        self.write_json(out, url, rev, hash, args)
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

pub fn fod_prefetch(expr: String) -> Result<String> {
    eprintln!("$ nix build --impure --no-link --expr '{expr}'");

    let Output {
        stdout,
        stderr,
        status,
    } = Command::new("nix")
        .arg("build")
        .arg("--impure")
        .arg("--no-link")
        .arg("--expr")
        .arg(expr)
        .output()?;

    if status.success() {
        bail!(
            "command succeeded unexpectedly\nstdout:\n{}",
            String::from_utf8_lossy(&stdout),
        );
    }

    let mut lines = stderr.lines();
    while let Some(line) = lines.next() {
        if !matches!(line, Ok(line) if line.trim_start().starts_with("specified:")) {
            continue;
        }
        let Some(line) = lines.next() else { break; };
        if let Ok(line) = line {
            let Some(hash) = line.trim_start().strip_prefix("got:") else { continue; };
            return Ok(hash.trim().into());
        }
    }

    Err(anyhow!(
        "failed to find the hash from error messages\nstdout: {}\nstderr:\n{}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr),
    ))
}
