use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
};

pub struct FetchFromGitea(pub String);
impl_fetcher!(FetchFromGitea);

impl<'a> SimpleFetcher<'a> for FetchFromGitea {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitea";

    fn host(&'a self) -> Option<&'a str> {
        Some(&self.0)
    }
}

impl<'a> SimpleUrlFetcher<'a> for FetchFromGitea {
    fn get_url(&self, [owner, repo]: [&str; 2], rev: &str) -> String {
        format!("https://{}/{owner}/{repo}/archive/{rev}.tar.gz", self.0)
    }
}
