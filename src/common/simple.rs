use anyhow::{Context, Result};
use indoc::writedoc;
use serde_json::json;
use url::Url;

use std::{fmt::Write as _, io::Write};

use crate::common::{flake_prefetch, fod_prefetch};

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
