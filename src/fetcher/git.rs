use anyhow::Result;

use crate::{
    GitScheme, Url,
    config::FetcherConfig,
    impl_fetcher,
    prefetch::git_prefetch,
    simple::{RevKey, SimpleFetcher},
};

pub struct Fetchgit(pub GitScheme);
impl_fetcher!(Fetchgit);

impl<'a> SimpleFetcher<'a, 1> for Fetchgit {
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchgit";
    const REV_KEY: RevKey = RevKey::RevOrTag;
    const SUBMODULES_DEFAULT: bool = true;
    const SUBMODULES_KEY: Option<&'static str> = Some("fetchSubmodules");

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([if matches!(self.0, GitScheme::Plus) {
            url.as_str().strip_prefix("git+")?
        } else {
            url.as_str()
        }])
    }
}

impl Fetchgit {
    fn fetch(
        &self,
        values @ [url]: &[&str; 1],
        rev_key: &'static str,
        rev: &str,
        submodules: bool,
        cfg: &FetcherConfig,
    ) -> Result<String> {
        if cfg.has_args() {
            self.fetch_fod(values, rev_key, rev, submodules, cfg)
        } else {
            git_prefetch(matches!(self.0, GitScheme::Yes), url, rev, !submodules)
        }
    }
}
