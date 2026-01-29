use std::fmt::Write;

use eyre::Result;

use crate::{Url, config::FetcherConfig, prefetch::fod_prefetch, revless::RevlessFetcher};

pub struct Fetchpatch2;

impl RevlessFetcher for Fetchpatch2 {
    const NAME: &'static str = "fetchpatch2";

    fn fetch(&self, url: &Url, cfg: &FetcherConfig) -> Result<String> {
        let mut expr = format!(
            r#"(import({}){{}}).fetchpatch2{{url="{url}";hash="sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";"#,
            cfg.nixpkgs,
        );

        for (key, value) in &cfg.args {
            write!(expr, "{key}={value};")?;
        }
        for (key, value) in &cfg.args_str {
            write!(expr, r#"{key}="{value}";"#)?;
        }

        expr.push('}');

        fod_prefetch(expr)
    }
}
