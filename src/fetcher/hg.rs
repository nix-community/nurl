use url::Url;

use crate::{
    impl_fetcher,
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

    fn get_flake_ref(&self, [url]: [&str; 1], rev: &str) -> String {
        format!(
            "hg+{url}?{}={rev}",
            if rev.len() == 40 { "rev" } else { "ref" },
        )
    }
}
