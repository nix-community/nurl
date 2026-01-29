use itertools::Itertools;
use rustc_hash::FxHashMap;
use serde_json::{Value, json};

use crate::cli::Opts;

pub struct FetcherConfig {
    pub rev: Option<String>,
    pub submodules: Option<bool>,
    pub nixpkgs: String,
    pub indent: usize,
    pub args: FxHashMap<String, String>,
    pub args_str: FxHashMap<String, String>,
    pub overwrites: FxHashMap<String, String>,
    pub overwrites_str: FxHashMap<String, String>,
    pub overwrite_rev: Option<String>,
    pub overwrite_rev_str: Option<String>,
}

impl FetcherConfig {
    pub fn has_args(&self) -> bool {
        !(self.args.is_empty() && self.args_str.is_empty())
    }

    pub fn merge_overwrites(&mut self) {
        for (key, value) in self.overwrites_str.drain() {
            self.overwrites.insert(key, format!(r#""{value}""#));
        }
        if let Some(rev) = self.overwrite_rev_str.take() {
            self.overwrite_rev = Some(format!(r#""{rev}""#));
        }
    }

    pub fn extend_fetcher_args(&self, fetcher_args: &mut Value, rev_key: &str) {
        for (key, value) in &self.args {
            fetcher_args[key] = json!({
                "type": "nix",
                "value": value,
            });
        }
        for (key, value) in &self.args_str {
            fetcher_args[key] = json!(value);
        }

        for (key, value) in &self.overwrites {
            fetcher_args[key] = json!({
                "type": "nix",
                "value": value,
            });
        }
        for (key, value) in &self.overwrites_str {
            fetcher_args[key] = json!(value);
        }
        if let Some(rev) = &self.overwrite_rev {
            fetcher_args[rev_key] = json!({
                "type": "nix",
                "value": rev,
            });
        }
        if let Some(rev) = &self.overwrite_rev_str {
            fetcher_args[rev_key] = json!(rev);
        }
    }
}

impl From<Opts> for FetcherConfig {
    fn from(opts: Opts) -> Self {
        Self {
            rev: opts.rev,
            submodules: opts.submodules,
            nixpkgs: opts.nixpkgs,
            indent: opts.indent,
            args: opts.args.into_iter().tuples().collect(),
            args_str: opts.args_str.into_iter().tuples().collect(),
            overwrites: opts.overwrites.into_iter().tuples().collect(),
            overwrites_str: opts.overwrites_str.into_iter().tuples().collect(),
            overwrite_rev: opts.overwrite_rev,
            overwrite_rev_str: opts.overwrite_rev_str,
        }
    }
}
