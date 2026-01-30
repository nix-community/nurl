use eyre::Result;

use crate::{Url, config::FetcherConfig, prefetch::url_prefetch, revless::RevlessFetcher};

pub struct Fetchzip;

impl RevlessFetcher for Fetchzip {
    const NAME: &'static str = "fetchzip";

    fn fetch(&self, url: &Url, cfg: &FetcherConfig) -> Result<String> {
        if cfg.has_args() {
            self.fetch_fod(url, cfg)
        } else {
            url_prefetch(url.as_str(), true)
        }
    }
}
