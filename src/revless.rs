use std::io::Write;

use eyre::{Result, bail};
use serde_json::json;

use crate::{Url, config::FetcherConfig, fetcher::Fetcher};

pub trait RevlessFetcher {
    const NAME: &'static str;

    fn fetch(&self, url: &Url) -> Result<String>;
}

impl<'a, T: RevlessFetcher> Fetcher<'a> for T {
    fn fetch_nix(&self, out: &mut impl Write, url: &'a Url, mut cfg: FetcherConfig) -> Result<()> {
        if cfg.has_rev() {
            bail!("{} does not support revisions", Self::NAME);
        }

        let hash = self.fetch(url)?;
        let indent = " ".repeat(cfg.indent);

        writeln!(out, "{} {{", Self::NAME)?;

        if let Some(url) = cfg.overwrites.remove("url") {
            writeln!(out, "{indent}  {url} = {url};")?;
        } else {
            writeln!(out, r#"{indent}  url = "{url}";"#)?;
        }

        if let Some(hash) = cfg.overwrites.remove("hash") {
            writeln!(out, "{indent}  {hash} = {hash};")?;
        } else {
            writeln!(out, r#"{indent}  hash = "{hash}";"#)?;
        }

        cfg.write_nix_args(out, &indent)?;
        write!(out, "{indent}}}")?;

        Ok(())
    }

    fn fetch_hash(&self, out: &mut impl Write, url: &'a Url, cfg: FetcherConfig) -> Result<()> {
        if cfg.has_rev() {
            bail!("{} does not support revisions", Self::NAME);
        }

        let hash = self.fetch(url)?;
        write!(out, "{hash}")?;

        Ok(())
    }

    fn fetch_json(&self, out: &mut impl Write, url: &'a Url, cfg: FetcherConfig) -> Result<()> {
        if cfg.has_rev() {
            bail!("{} does not support revisions", Self::NAME);
        }

        let hash = self.fetch(url)?;

        let mut fetcher_args = json!({
            "url": url.as_str(),
            "hash": hash,
        });

        cfg.extend_fetcher_args(&mut fetcher_args, "");

        serde_json::to_writer(
            out,
            &json!({
                "fetcher": Self::NAME,
                "args": fetcher_args,
            }),
        )?;

        Ok(())
    }

    fn to_json(&'a self, out: &mut impl Write, url: &'a Url, rev: Option<String>) -> Result<()> {
        if rev.is_some() {
            bail!("{} does not support revisions", Self::NAME);
        }

        serde_json::to_writer(
            out,
            &json!({
                "fetcher": Self::NAME,
                "args": {
                    "url": url.as_str(),
                },
            }),
        )?;

        Ok(())
    }
}
