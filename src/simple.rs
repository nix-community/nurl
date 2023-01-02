use anyhow::{Context, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use serde_json::json;
use url::Url;

use std::{fmt::Write as _, io::Write};

use crate::prefetch::{flake_prefetch, fod_prefetch, url_prefetch};

pub trait SimpleFetcher<'a, const N: usize> {
    const HOST_KEY: &'static str = "domain";
    const KEYS: [&'static str; N];
    const NAME: &'static str;

    fn host(&'a self) -> Option<&'a str> {
        None
    }

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; N]> {
        let mut xs: [_; N] = url
            .path_segments()?
            .chunks(N)
            .into_iter()
            .next()?
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;
        xs[N - 1] = xs[N - 1].strip_suffix(".git").unwrap_or(xs[N - 1]);
        Some(xs)
    }

    fn fetch_fod(
        &'a self,
        url: &'a Url,
        rev: &str,
        args: &[(String, String)],
        args_str: &[(String, String)],
    ) -> Result<([&str; N], String)> {
        let values = self
            .get_values(url)
            .with_context(|| format!("failed to parse {url}"))?;

        let mut expr = format!(r#"(import <nixpkgs> {{}}).{}{{"#, Self::NAME);

        if let Some(host) = self.host() {
            write!(expr, r#"{}="{host}""#, Self::HOST_KEY)?;
        }

        for (key, value) in Self::KEYS.iter().zip(values) {
            write!(expr, r#"{key}="{value}";"#)?;
        }

        write!(
            expr,
            r#"rev="{rev}";hash="sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";"#
        )?;

        for (key, value) in args {
            write!(expr, "{key}={value};")?;
        }
        for (key, value) in args_str {
            write!(expr, r#"{key}="{value}";"#)?;
        }

        expr.push('}');

        let hash = fod_prefetch(expr)?;

        Ok((values, hash))
    }

    fn write_nix(
        &'a self,
        out: &mut impl Write,
        values: [&str; N],
        rev: String,
        hash: String,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
        overwrites: FxHashMap<String, String>,
        indent: String,
    ) -> Result<()> {
        let mut overwrites = overwrites;

        writeln!(out, "{} {{", Self::NAME)?;

        if let Some(host) = overwrites.remove(Self::HOST_KEY) {
            writeln!(out, r#"{indent}  {} = {host};"#, Self::HOST_KEY)?;
        } else if let Some(host) = self.host() {
            writeln!(out, r#"{indent}  {} = "{host}";"#, Self::HOST_KEY)?;
        }

        for (key, value) in Self::KEYS.iter().zip(values) {
            if let Some(value) = overwrites.remove(*key) {
                writeln!(out, r#"{indent}  {key} = {value};"#)?;
            } else {
                writeln!(out, r#"{indent}  {key} = "{value}";"#)?;
            }
        }

        if let Some(rev) = overwrites.remove("rev") {
            writeln!(out, "{indent}  rev = {rev};")?;
        } else {
            writeln!(out, r#"{indent}  rev = "{rev}";"#)?;
        }
        if let Some(hash) = overwrites.remove("hash") {
            writeln!(out, "{indent}  hash = {hash};")?;
        } else {
            writeln!(out, r#"{indent}  hash = "{hash}";"#)?;
        }

        for (key, value) in args {
            let value = overwrites.remove(&key).unwrap_or(value);
            writeln!(out, "{indent}  {key} = {value};")?;
        }
        for (key, value) in args_str {
            if let Some(value) = overwrites.remove(&key) {
                writeln!(out, "{indent}  {key} = {value};")?;
            } else {
                writeln!(out, r#"{indent}  {key} = "{value}";"#)?;
            }
        }

        for (key, value) in overwrites {
            writeln!(out, "{indent}  {key} = {value};")?;
        }

        write!(out, "{indent}}}")?;

        Ok(())
    }

    fn write_json(
        &'a self,
        out: &mut impl Write,
        values: [&str; N],
        rev: String,
        hash: String,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
        overwrites: Vec<(String, String)>,
        overwrites_str: Vec<(String, String)>,
    ) -> Result<()> {
        let mut fetcher_args = json!({
            "rev": rev,
            "hash": hash,
        });

        if let Some(host) = self.host() {
            fetcher_args["host"] = json!(host);
        }

        for (key, value) in Self::KEYS.iter().zip(values) {
            fetcher_args[key] = json!(value);
        }

        for (key, value) in args {
            fetcher_args[key] = json!({
                "type": "nix",
                "value": value,
            });
        }
        for (key, value) in args_str {
            fetcher_args[key] = json!(value);
        }

        for (key, value) in overwrites {
            fetcher_args[key] = json!({
                "type": "nix",
                "value": value,
            })
        }
        for (key, value) in overwrites_str {
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

pub trait SimpleFodFetcher<'a, const N: usize>: SimpleFetcher<'a, N> {
    fn fetch(
        &'a self,
        url: &'a Url,
        rev: &str,
        args: &[(String, String)],
        args_str: &[(String, String)],
    ) -> Result<([&str; N], String)> {
        self.fetch_fod(url, rev, args, args_str)
    }
}

pub trait SimpleFlakeFetcher<'a, const N: usize>: SimpleFetcher<'a, N> {
    fn get_flake_ref(&'a self, values: [&str; N], rev: &str) -> String;

    fn fetch(
        &'a self,
        url: &'a Url,
        rev: &str,
        args: &[(String, String)],
        args_str: &[(String, String)],
    ) -> Result<([&str; N], String)> {
        if args.is_empty() && args_str.is_empty() {
            let values = self
                .get_values(url)
                .with_context(|| format!("failed to parse {url}"))?;
            let hash = flake_prefetch(self.get_flake_ref(values, rev))?;
            Ok((values, hash))
        } else {
            self.fetch_fod(url, rev, args, args_str)
        }
    }
}

pub trait SimpleUrlFetcher<'a, const N: usize>: SimpleFetcher<'a, N> {
    fn get_url(&self, values: [&str; N], rev: &str) -> String;

    fn fetch(
        &'a self,
        url: &'a Url,
        rev: &str,
        args: &[(String, String)],
        args_str: &[(String, String)],
    ) -> Result<([&str; N], String)> {
        if args.is_empty() && args_str.is_empty() {
            let values = self
                .get_values(url)
                .with_context(|| format!("failed to parse {url}"))?;
            let hash = url_prefetch(self.get_url(values, rev))?;
            Ok((values, hash))
        } else {
            self.fetch_fod(url, rev, args, args_str)
        }
    }
}
