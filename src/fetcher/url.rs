use eyre::Result;

use crate::{Url, config::FetcherConfig, prefetch::url_prefetch, revless::RevlessFetcher};

pub struct Fetchurl;

impl RevlessFetcher for Fetchurl {
    const NAME: &'static str = "fetchurl";

    fn fetch(&self, url: &Url, _: &FetcherConfig) -> Result<String> {
        url_prefetch(url.as_str(), false)
    }
}
