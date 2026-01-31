use eyre::Result;

use crate::{Url, config::FetcherConfig, prefetch::url_prefetch, revless::RevlessFetcher};

pub struct Fetchurl;

impl RevlessFetcher for Fetchurl {
    const NAME: &'static str = "fetchurl";

    fn fetch(&self, url: &Url, cfg: &FetcherConfig) -> Result<String> {
        if cfg.has_args() {
            self.fetch_fod(url, cfg)
        } else {
            url_prefetch(url.as_str())
        }
    }
}
