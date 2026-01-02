use anyhow::Result;

use crate::{
    GitScheme, Url, impl_fetcher,
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
        args: &[(String, String)],
        args_str: &[(String, String)],
        nixpkgs: String,
    ) -> Result<String> {
        if args.is_empty() && args_str.is_empty() {
            git_prefetch(matches!(self.0, GitScheme::Yes), url, rev, !submodules)
        } else {
            self.fetch_fod(values, rev_key, rev, submodules, args, args_str, nixpkgs)
        }
    }
}
