use url::Url;

use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
    GitScheme,
};

pub struct Fetchgit(pub GitScheme);
impl_fetcher!(Fetchgit);

impl<'a> SimpleFetcher<'a, 1> for Fetchgit {
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchgit";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([if matches!(self.0, GitScheme::Plus) {
            url.as_ref().strip_prefix("git+")?
        } else {
            url.as_ref()
        }])
    }
}

impl<'a> SimpleFlakeFetcher<'a, 1> for Fetchgit {
    fn get_flake_ref(&self, [url]: &[&str; 1], rev: &str) -> String {
        let rev_type = if rev.len() == 40 { "rev" } else { "ref" };
        if matches!(self.0, GitScheme::Yes) {
            format!("{url}?{rev_type}={rev}&submodules=1")
        } else {
            format!("git+{url}?{rev_type}={rev}&submodules=1")
        }
    }
}
