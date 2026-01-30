use eyre::Result;

use crate::{Url, config::FetcherConfig, revless::RevlessFetcher};

pub struct Fetchpatch2;

impl RevlessFetcher for Fetchpatch2 {
    const NAME: &'static str = "fetchpatch2";

    fn fetch(&self, url: &Url, cfg: &FetcherConfig) -> Result<String> {
        self.fetch_fod(url, cfg)
    }
}
