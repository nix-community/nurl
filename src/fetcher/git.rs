use anyhow::Result;
use url::Url;

use crate::{
    impl_fetcher,
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

    fn get_flake_ref(&self, [url]: [&str; 1], rev: &str) -> Result<String> {
        Ok(format!(
            "git+{url}?{}={rev}&submodules=1",
            if rev.len() == 40 { "rev" } else { "ref" },
        ))
    }
}
