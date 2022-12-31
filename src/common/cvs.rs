use anyhow::Result;
use indoc::writedoc;
use serde_json::json;
use url::Url;

use std::{fmt::Write as _, io::Write};

use crate::common::{flake_prefetch, fod_prefetch};

pub trait CvsFetcher {
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

pub trait CvsFodFetcher: CvsFetcher {
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

pub trait CvsFlakeFetcher: CvsFetcher {
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
