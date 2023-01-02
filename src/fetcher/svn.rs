use url::Url;

use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFodFetcher},
};

pub struct Fetchsvn;
impl_fetcher!(Fetchsvn);

impl<'a> SimpleFetcher<'a, 1> for Fetchsvn {
    const HASH_KEY: &'static str = "sha256";
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchsvn";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([url.as_ref()])
    }
}

impl<'a> SimpleFodFetcher<'a, 1> for Fetchsvn {}
