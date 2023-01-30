use anyhow::Result;

use crate::{impl_fetcher, prefetch::git_prefetch, simple::SimpleFetcher, GitScheme, Url};

pub struct Fetchgit(pub GitScheme);
impl_fetcher!(Fetchgit);

impl<'a> SimpleFetcher<'a, 1> for Fetchgit {
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchgit";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([if matches!(self.0, GitScheme::Plus) {
            url.as_str().strip_prefix("git+")?
        } else {
            url.as_str()
        }])
    }
}

impl<'a> Fetchgit {
    fn fetch(
        &'a self,
        values @ [url]: &[&str; 1],
        rev: &str,
        args: &[(String, String)],
        args_str: &[(String, String)],
    ) -> Result<String> {
        if args.is_empty() && args_str.is_empty() {
            git_prefetch(matches!(self.0, GitScheme::Yes), url, rev)
        } else {
            self.fetch_fod(values, rev, args, args_str)
        }
    }
}
