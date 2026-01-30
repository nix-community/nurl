use eyre::Result;

use crate::{Url, config::FetcherConfig, revless::RevlessFetcher};

pub struct Fetchpatch;

impl RevlessFetcher for Fetchpatch {
    const NAME: &'static str = "fetchpatch";

    fn fetch(&self, url: &Url, cfg: &FetcherConfig) -> Result<String> {
        self.fetch_fod(url, cfg)
    }
}
