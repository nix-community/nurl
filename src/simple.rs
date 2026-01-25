use std::{fmt::Write as _, io::Write};

use anyhow::{Result, bail};
use itertools::Itertools;
use serde_json::{Value, json};

use crate::{
    Url,
    config::FetcherConfig,
    prefetch::{flake_prefetch, fod_prefetch, git_prefetch, url_prefetch},
};

pub enum RevKey {
    Const(&'static str),
    RevOrTag,
}

pub trait SimpleFetcher<'a, const N: usize> {
    const HASH_KEY: &'static str = "hash";
    const HOST_KEY: &'static str = "domain";
    const KEYS: [&'static str; N];
    const NAME: &'static str;
    const REV_KEY: RevKey = RevKey::Const("rev");
    const SUBMODULES_DEFAULT: bool = false;
    const SUBMODULES_KEY: Option<&'static str> = None;

    fn rev_entry<'b>(&self, rev: &'b str) -> (&'static str, &'b str) {
        match Self::REV_KEY {
            RevKey::Const(rev_key) => (rev_key, rev),
            RevKey::RevOrTag => (
                if rev.len() == 40 { "rev" } else { "tag" },
                rev.strip_prefix("refs/tags/").unwrap_or(rev),
            ),
        }
    }

    fn host(&self) -> Option<&str> {
        None
    }

    fn group(&self) -> Option<&str> {
        None
    }

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; N]> {
        let mut xs: [_; N] = url
            .path_segments()
            .chunks(N)
            .into_iter()
            .next()?
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;
        xs[N - 1] = xs[N - 1].strip_suffix(".git").unwrap_or(xs[N - 1]);
        Some(xs)
    }

    fn resolve_submodules(&self, submodules: Option<bool>) -> bool {
        submodules.is_some_and(|submodules| submodules ^ Self::SUBMODULES_DEFAULT)
    }

    fn fetch_rev(&self, _: &[&str; N]) -> Result<String> {
        bail!(
            "{} does not support fetching the latest revision",
            Self::NAME,
        );
    }

    fn fetch_fod(
        &self,
        values: &[&str; N],
        rev_key: &'static str,
        rev: &str,
        submodules: bool,
        cfg: &FetcherConfig,
    ) -> Result<String> {
        let mut expr = format!(r#"(import({}){{}}).{}{{"#, cfg.nixpkgs, Self::NAME);

        if let Some(host) = self.host() {
            write!(expr, r#"{}="{host}";"#, Self::HOST_KEY)?;
        }

        if let Some(group) = self.group() {
            write!(expr, r#"group="{group}";"#)?;
        }

        for (key, value) in Self::KEYS.iter().zip(values) {
            write!(expr, r#"{key}="{value}";"#)?;
        }

        write!(
            expr,
            r#"{rev_key}="{rev}";{}="sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";"#,
            Self::HASH_KEY,
        )?;

        if submodules && let Some(key) = Self::SUBMODULES_KEY {
            write!(expr, "{key}={};", !Self::SUBMODULES_DEFAULT)?;
        }

        for (key, value) in &cfg.args {
            write!(expr, "{key}={value};")?;
        }
        for (key, value) in &cfg.args_str {
            write!(expr, r#"{key}="{value}";"#)?;
        }

        expr.push('}');

        fod_prefetch(expr)
    }

    #[allow(clippy::too_many_arguments)]
    fn write_nix(
        &self,
        out: &mut impl Write,
        values: &[&str; N],
        rev_key: &'static str,
        rev: &str,
        hash: String,
        submodules: bool,
        mut cfg: FetcherConfig,
    ) -> Result<()> {
        let indent = " ".repeat(cfg.indent);

        writeln!(out, "{} {{", Self::NAME)?;

        if let Some(host) = cfg.overwrites.remove(Self::HOST_KEY) {
            writeln!(out, r#"{indent}  {} = {host};"#, Self::HOST_KEY)?;
        } else if let Some(host) = self.host() {
            writeln!(out, r#"{indent}  {} = "{host}";"#, Self::HOST_KEY)?;
        }

        if let Some(group) = cfg.overwrites.remove("group") {
            writeln!(out, r#"{indent}  group = {group};"#)?;
        } else if let Some(group) = self.group() {
            writeln!(out, r#"{indent}  group = "{group}";"#)?;
        }

        for (key, value) in Self::KEYS.iter().zip(values) {
            if let Some(value) = cfg.overwrites.remove(*key) {
                writeln!(out, r#"{indent}  {key} = {value};"#)?;
            } else {
                writeln!(out, r#"{indent}  {key} = "{value}";"#)?;
            }
        }

        if let Some(rev) = cfg.overwrites.remove(rev_key) {
            writeln!(out, "{indent}  {rev_key} = {rev};")?;
        } else {
            writeln!(out, r#"{indent}  {rev_key} = "{rev}";"#)?;
        }
        if let Some(hash) = cfg.overwrites.remove(Self::HASH_KEY) {
            writeln!(out, "{indent}  {} = {hash};", Self::HASH_KEY)?;
        } else {
            writeln!(out, r#"{indent}  {} = "{hash}";"#, Self::HASH_KEY)?;
        }

        if let Some(key) = Self::SUBMODULES_KEY {
            if let Some(submodules) = cfg.overwrites.remove(key) {
                writeln!(out, "{indent}  {key} = {submodules};")?;
            } else if submodules {
                writeln!(out, "{indent}  {key} = {};", !Self::SUBMODULES_DEFAULT)?;
            }
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

    #[allow(clippy::too_many_arguments)]
    fn write_json(
        &self,
        out: &mut impl Write,
        values: &[&str; N],
        rev_key: &'static str,
        rev: &str,
        hash: String,
        submodules: bool,
        cfg: FetcherConfig,
    ) -> Result<()> {
        let mut fetcher_args = Value::from_iter(
            Self::KEYS
                .into_iter()
                .zip(*values)
                .chain([(rev_key, rev), (Self::HASH_KEY, hash.as_ref())]),
        );

        if let Some(host) = self.host() {
            fetcher_args[Self::HOST_KEY] = json!(host);
        }

        if let Some(group) = self.group() {
            fetcher_args["group"] = json!(group);
        }

        if submodules && let Some(key) = Self::SUBMODULES_KEY {
            fetcher_args[key] = json!(!Self::SUBMODULES_DEFAULT);
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
                "fetcher": Self::NAME,
                "args": fetcher_args,
            }),
        )?;

        Ok(())
    }
}

pub trait SimpleFodFetcher<'a, const N: usize>: SimpleFetcher<'a, N> {
    fn fetch(
        &self,
        values: &[&str; N],
        rev_key: &'static str,
        rev: &str,
        submodules: bool,
        cfg: &FetcherConfig,
    ) -> Result<String> {
        self.fetch_fod(values, rev_key, rev, submodules, cfg)
    }
}

pub trait SimpleGitFetcher<'a, const N: usize>: SimpleFetcher<'a, N> {
    fn get_flake_ref(&self, values: &[&str; N], rev: &str) -> String;

    fn get_repo_url(&self, values: &[&str; N]) -> String;

    fn fetch(
        &self,
        values: &[&str; N],
        rev_key: &'static str,
        rev: &str,
        submodules: bool,
        cfg: &FetcherConfig,
    ) -> Result<String> {
        if cfg.has_args() {
            self.fetch_fod(values, rev_key, rev, submodules, cfg)
        } else if submodules {
            git_prefetch(
                true,
                &self.get_repo_url(values),
                rev,
                !Self::SUBMODULES_DEFAULT,
            )
        } else {
            flake_prefetch(self.get_flake_ref(values, rev))
        }
    }
}

pub trait SimpleUrlFetcher<'a, const N: usize>: SimpleFetcher<'a, N> {
    const UNPACK: bool = true;

    fn get_url(&self, values: &[&str; N], rev: &str) -> String;

    fn fetch(
        &self,
        values: &[&str; N],
        rev_key: &'static str,
        rev: &str,
        submodules: bool,
        cfg: &FetcherConfig,
    ) -> Result<String> {
        if cfg.has_args() {
            self.fetch_fod(values, rev_key, rev, submodules, cfg)
        } else {
            url_prefetch(self.get_url(values, rev), Self::UNPACK)
        }
    }
}
