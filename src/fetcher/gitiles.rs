use url::Url;

use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
};

pub struct FetchFromGitiles;
impl_fetcher!(FetchFromGitiles);

impl<'a> SimpleFetcher<'a, 1> for FetchFromGitiles {
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchFromGitiles";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([url.as_ref()])
    }
}

impl<'a> SimpleUrlFetcher<'a, 1> for FetchFromGitiles {
    fn get_url(&self, [url]: &[&str; 1], rev: &str) -> String {
        format!("{url}/+archive/{rev}.tar.gz")
    }
}
