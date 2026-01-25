use itertools::Itertools;
use rustc_hash::FxHashMap;

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
}

impl FetcherConfig {
    pub fn has_args(&self) -> bool {
        !(self.args.is_empty() && self.args_str.is_empty())
    }

    pub fn merge_overwrites(&mut self) {
        for (key, value) in self.overwrites_str.drain() {
            self.overwrites.insert(key, format!(r#""{value}""#));
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
        }
    }
}
