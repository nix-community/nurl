use std::io::Write;

use eyre::{OptionExt, Result, bail};
use serde_json::json;

use crate::{Url, config::FetcherConfig, fetcher::Fetcher};

pub struct BuiltinsFetchGit;

impl<'a> Fetcher<'a> for BuiltinsFetchGit {
    fn fetch_nix(&self, out: &mut impl Write, url: &'a Url, mut cfg: FetcherConfig) -> Result<()> {
        let indent = " ".repeat(cfg.indent);
        let rev = cfg
            .rev
            .ok_or_eyre("builtins.fetchGit does not support feching the latest revision")?;
        let rev_type = if rev.len() == 40 { "rev" } else { "ref" };

        writeln!(out, "builtins.fetchGit {{")?;

        if let Some(url) = cfg.overwrites.remove("url") {
            writeln!(out, "{indent}  {url} = {url};")?;
        } else {
            writeln!(out, r#"{indent}  url = "{url}";"#)?;
        }

        if let Some(rev) = cfg.overwrites.remove(rev_type) {
            writeln!(out, "{indent}  {rev_type} = {rev};")?;
        } else {
            writeln!(out, r#"{indent}  {rev_type} = "{rev}";"#)?;
        }

        if let Some(submodules) = cfg.overwrites.remove("submodules") {
            writeln!(out, "{indent}  submodules = {submodules};")?;
        } else if matches!(cfg.submodules, Some(true)) {
            writeln!(out, "{indent}  submodules = true;")?;
        }

        for (key, value) in cfg.args {
            let value = cfg.overwrites.remove(&key).unwrap_or(value);
            writeln!(out, "{indent}  {key} = {value};")?;
        }
        for (key, value) in cfg.args_str {
            if let Some(value) = cfg.overwrites.remove(&key) {
                writeln!(out, "{indent}  {key} = {value};")?;
            } else {
                writeln!(out, r#"{indent}  {key} = "{value}";"#)?;
            }
        }

        for (key, value) in cfg.overwrites {
            writeln!(out, "{indent}  {key} = {value};")?;
        }

        write!(out, "{indent}}}")?;

        Ok(())
    }

    fn fetch_hash(&self, _: &mut impl Write, _: &'a Url, _: FetcherConfig) -> Result<()> {
        bail!("builtins.fetchGit does not support hashes");
    }

    fn fetch_json(&self, out: &mut impl Write, url: &'a Url, cfg: FetcherConfig) -> Result<()> {
        let rev = cfg
            .rev
            .ok_or_eyre("builtins.fetchGit does not support feching the latest revision")?;
        let rev_type = if rev.len() == 40 { "rev" } else { "ref" };

        let mut fetcher_args = json!({
            "url": url.as_str(),
            rev_type: rev,
        });

        if matches!(cfg.submodules, Some(true)) {
            fetcher_args["submodules"] = json!(true);
        }

        for (key, value) in cfg.args {
            fetcher_args[key] = json!({
                "type": "nix",
                "value": value,
            });
        }
        for (key, value) in cfg.args_str {
            fetcher_args[key] = json!(value);
        }

        for (key, value) in cfg.overwrites {
            fetcher_args[key] = json!({
                "type": "nix",
                "value": value,
            })
        }
        for (key, value) in cfg.overwrites_str {
            fetcher_args[key] = json!(value);
        }

        serde_json::to_writer(
            out,
            &json!({
                "fetcher": "builtins.fetchGit",
                "args": fetcher_args,
            }),
        )?;

        Ok(())
    }

    fn to_json(&'a self, out: &mut impl Write, url: &'a Url, rev: Option<String>) -> Result<()> {
        let rev =
            rev.ok_or_eyre("builtins.fetchGit does not support feching the latest revision")?;
        let rev_type = if rev.len() == 40 { "rev" } else { "ref" };

        serde_json::to_writer(
            out,
            &json!({
                "fetcher": "builtins.fetchGit",
                "args": {
                    "url": url.as_str(),
                    rev_type: rev,
                },
            }),
        )?;

        Ok(())
    }
}
