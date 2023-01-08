use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
};

pub struct FetchFromGitea<'a>(pub &'a str);
impl_fetcher!(FetchFromGitea<'a>);

impl<'a> SimpleFetcher<'a, 2> for FetchFromGitea<'a> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitea";

    fn host(&'a self) -> Option<&'a str> {
        Some(self.0)
    }
}

impl<'a> SimpleUrlFetcher<'a, 2> for FetchFromGitea<'a> {
    fn get_url(&self, [owner, repo]: &[&str; 2], rev: &str) -> String {
        format!("https://{}/{owner}/{repo}/archive/{rev}.tar.gz", self.0)
    }
}
