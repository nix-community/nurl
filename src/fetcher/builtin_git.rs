use anyhow::{bail, Context, Result};
use rustc_hash::FxHashMap;
use serde_json::json;

use std::io::Write;

use crate::{fetcher::Fetcher, Url};

pub struct BuiltinsFetchGit;

impl<'a> Fetcher<'a> for BuiltinsFetchGit {
    fn fetch_nix(
        &self,
        out: &mut impl Write,
        url: &'a Url,
        rev: Option<String>,
        submodules: Option<bool>,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
        overwrites: FxHashMap<String, String>,
        _: String,
        indent: String,
    ) -> Result<()> {
        let mut overwrites = overwrites;
        let rev = rev.context("builtins.fetchGit does not support feching the latest revision")?;
        let rev_type = if rev.len() == 40 { "rev" } else { "ref" };

        writeln!(out, "builtins.fetchGit {{")?;

        if let Some(url) = overwrites.remove("url") {
            writeln!(out, "{indent}  {url} = {url};")?;
        } else {
            writeln!(out, r#"{indent}  url = "{url}";"#)?;
        }

        if let Some(rev) = overwrites.remove(rev_type) {
            writeln!(out, "{indent}  {rev_type} = {rev};")?;
        } else {
            writeln!(out, r#"{indent}  {rev_type} = "{rev}";"#)?;
        }

        if let Some(submodules) = overwrites.remove("submodules") {
            writeln!(out, "{indent}  submodules = {submodules};")?;
        } else if matches!(submodules, Some(true)) {
            writeln!(out, "{indent}  submodules = true;")?;
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

    fn fetch_hash(
        &self,
        _: &mut impl Write,
        _: &'a Url,
        _: Option<String>,
        _: Option<bool>,
        _: Vec<(String, String)>,
        _: Vec<(String, String)>,
        _: String,
    ) -> Result<()> {
        bail!("builtins.fetchGit does not support hashes");
    }

    fn fetch_json(
        &self,
        out: &mut impl Write,
        url: &'a Url,
        rev: Option<String>,
        submodules: Option<bool>,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
        overwrites: Vec<(String, String)>,
        overwrites_str: Vec<(String, String)>,
        _: String,
    ) -> Result<()> {
        let rev = rev.context("builtins.fetchGit does not support feching the latest revision")?;
        let rev_type = if rev.len() == 40 { "rev" } else { "ref" };

        let mut fetcher_args = json!({
            "url": url.to_string(),
            rev_type: rev,
        });

        if matches!(submodules, Some(true)) {
            fetcher_args["submodules"] = json!(true);
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
                "fetcher": "builtins.fetchGit",
                "args": fetcher_args,
            }),
        )?;

        Ok(())
    }

    fn to_json(&'a self, out: &mut impl Write, url: &'a Url, rev: Option<String>) -> Result<()> {
        let rev = rev.context("builtins.fetchGit does not support feching the latest revision")?;
        let rev_type = if rev.len() == 40 { "rev" } else { "ref" };

        serde_json::to_writer(
            out,
            &json!({
                "fetcher": "builtins.fetchGit",
                "args": {
                    "url": url.to_string(),
                    rev_type: rev,
                },
            }),
        )?;

        Ok(())
    }
}
