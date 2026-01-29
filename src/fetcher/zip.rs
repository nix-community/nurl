use eyre::Result;

use crate::{Url, config::FetcherConfig, prefetch::url_prefetch, revless::RevlessFetcher};

pub struct Fetchzip;

impl RevlessFetcher for Fetchzip {
    const NAME: &'static str = "fetchzip";

    fn fetch(&self, url: &Url, _: &FetcherConfig) -> Result<String> {
        url_prefetch(url.as_str(), true)
    }
}
