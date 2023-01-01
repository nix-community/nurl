use anyhow::{Context, Result};
use indoc::writedoc;
use itertools::Itertools;
use serde_json::json;
use url::Url;

use std::{fmt::Write as _, io::Write};

use crate::prefetch::{flake_prefetch, fod_prefetch, url_prefetch};

pub trait SimpleFetcher<'a, const N: usize = 2> {
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
        indent: String,
    ) -> Result<()> {
        writeln!(out, "{} {{", Self::NAME)?;

        if let Some(host) = self.host() {
            writeln!(out, r#"{indent}  {} = "{host}";"#, Self::HOST_KEY)?;
        }

        for (key, value) in Self::KEYS.iter().zip(values) {
            writeln!(out, r#"{indent}  {key} = "{value}";"#)?;
        }

        writedoc!(
            out,
            r#"
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
        values: [&str; N],
        rev: String,
        hash: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let mut fetcher_args = json! ({
            "rev": rev,
            "hash": hash,
        });

        if let Some(host) = self.host() {
            fetcher_args["host"] = json!(host);
        }

        for (key, value) in Self::KEYS
            .into_iter()
            .zip(values)
            .chain(args.iter().map(|(x, y)| (x.as_str(), y.as_str())))
        {
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

pub trait SimpleFodFetcher<'a, const N: usize = 2>: SimpleFetcher<'a, N> {
    fn fetch_nix_impl(
        &'a self,
        out: &mut impl Write,
        url: &'a Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
        let (values, hash) = self.fetch_fod(url, &rev, &args)?;
        self.write_nix(out, values, rev, hash, args, indent)
    }

    fn fetch_json_impl(
        &'a self,
        out: &mut impl Write,
        url: &'a Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let (values, hash) = self.fetch_fod(url, &rev, &args)?;
        self.write_json(out, values, rev, hash, args)
    }
}

pub trait SimpleFlakeFetcher<'a, const N: usize = 2>: SimpleFetcher<'a, N> {
    const FLAKE_TYPE: &'static str;

    fn fetch(&'a self, url: &'a Url, rev: &str) -> Result<([&str; N], String)> {
        let values = self
            .get_values(url)
            .with_context(|| format!("failed to parse {url} as a {} url", Self::FLAKE_TYPE))?;

        let mut flake_ref = format!("{}:", Self::FLAKE_TYPE);
        for value in values {
            flake_ref.push_str(value);
            flake_ref.push('/');
        }
        flake_ref.push_str(rev);

        if let Some(host) = self.host() {
            flake_ref.push_str("?host=");
            flake_ref.push_str(host);
        }

        let hash = flake_prefetch(flake_ref)?;

        Ok((values, hash))
    }

    fn fetch_nix_impl(
        &'a self,
        out: &mut impl Write,
        url: &'a Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
        let (values, hash) = if args.is_empty() {
            self.fetch(url, &rev)?
        } else {
            self.fetch_fod(url, &rev, &args)?
        };

        self.write_nix(out, values, rev, hash, args, indent)
    }

    fn fetch_json_impl(
        &'a self,
        out: &mut impl Write,
        url: &'a Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let (values, hash) = if args.is_empty() {
            self.fetch(url, &rev)?
        } else {
            self.fetch_fod(url, &rev, &args)?
        };
        self.write_json(out, values, rev, hash, args)
    }
}

pub trait SimpleUrlFetcher<'a, const N: usize = 2>: SimpleFetcher<'a, N> {
    fn get_url(&self, values: [&str; N], rev: &str) -> String;

    fn fetch(&'a self, url: &'a Url, rev: &str) -> Result<([&str; N], String)> {
        let values = self
            .get_values(url)
            .with_context(|| format!("failed to parse {url}"))?;
        let hash = url_prefetch(self.get_url(values, rev))?;
        Ok((values, hash))
    }

    fn fetch_nix_impl(
        &'a self,
        out: &mut impl Write,
        url: &'a Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()> {
        let (values, hash) = if args.is_empty() {
            self.fetch(url, &rev)?
        } else {
            self.fetch_fod(url, &rev, &args)?
        };

        self.write_nix(out, values, rev, hash, args, indent)
    }

    fn fetch_json_impl(
        &'a self,
        out: &mut impl Write,
        url: &'a Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()> {
        let (values, hash) = if args.is_empty() {
            self.fetch(url, &rev)?
        } else {
            self.fetch_fod(url, &rev, &args)?
        };
        self.write_json(out, values, rev, hash, args)
    }
}
