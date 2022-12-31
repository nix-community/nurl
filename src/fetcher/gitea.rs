use crate::{
    common::{SimpleFetcher, SimpleUrlFetcher},
    impl_fetcher,
};

pub struct FetchFromGitea(pub String);
impl_fetcher!(FetchFromGitea);

impl<'a> SimpleFetcher<'a> for FetchFromGitea {
    const NAME: &'static str = "fetchFromGitea";

    fn host(&'a self) -> Option<&'a str> {
        Some(&self.0)
    }
}

impl<'a> SimpleUrlFetcher<'a> for FetchFromGitea {
    fn get_url(&self, owner: &str, repo: &str, rev: &str) -> String {
        format!("https://{}/{owner}/{repo}/archive/{rev}.tar.gz", self.0)
    }
}
