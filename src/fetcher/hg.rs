use url::Url;

use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct Fetchhg(pub bool);
impl_fetcher!(Fetchhg);

impl<'a> SimpleFetcher<'a, 1> for Fetchhg {
    const HASH_KEY: &'static str = "sha256";
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchhg";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([if self.0 {
            url.as_ref().strip_prefix("hg+")?
        } else {
            url.as_ref()
        }])
    }
}

impl<'a> SimpleFlakeFetcher<'a, 1> for Fetchhg {
    fn get_flake_ref(&self, [url]: &[&str; 1], rev: &str) -> String {
        format!(
            "hg+{url}?{}={rev}",
            if rev.len() == 40 { "rev" } else { "ref" },
        )
    }
}
