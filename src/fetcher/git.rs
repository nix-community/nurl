use anyhow::Result;
use url::Url;

use crate::{
    impl_fetcher,
    prefetch::flake_prefetch,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct Fetchgit;
impl_fetcher!(Fetchgit);

impl<'a> SimpleFetcher<'a, 1> for Fetchgit {
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchgit";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([url.as_ref()])
    }
}

impl<'a> SimpleFlakeFetcher<'a, 1> for Fetchgit {
    const FLAKE_TYPE: &'static str = "git";

    fn fetch(&'a self, url: &'a Url, rev: &str) -> Result<([&'a str; 1], String)> {
        Ok((
            [url.as_ref()],
            flake_prefetch(format!(
                "{}+{url}?{}={rev}",
                Self::FLAKE_TYPE,
                if rev.len() == 40 { "rev" } else { "ref" },
            ))?,
        ))
    }
}
