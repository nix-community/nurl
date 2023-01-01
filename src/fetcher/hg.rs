use anyhow::Result;
use url::Url;

use crate::{
    impl_fetcher,
    prefetch::flake_prefetch,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct Fetchhg;
impl_fetcher!(Fetchhg);

impl<'a> SimpleFetcher<'a, 1> for Fetchhg {
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchhg";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([url.as_ref()])
    }
}

impl<'a> SimpleFlakeFetcher<'a, 1> for Fetchhg {
    const FLAKE_TYPE: &'static str = "hg";

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
