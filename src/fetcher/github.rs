use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct FetchFromGitHub<'a>(pub Option<&'a str>);
impl_fetcher!(FetchFromGitHub<'a>);

impl<'a> SimpleFetcher<'a, 2> for FetchFromGitHub<'a> {
    const HOST_KEY: &'static str = "githubBase";
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitHub";

    fn host(&'a self) -> Option<&'a str> {
        self.0
    }
}

impl<'a> SimpleFlakeFetcher<'a, 2> for FetchFromGitHub<'a> {
    fn get_flake_ref(&'a self, [owner, repo]: [&str; 2], rev: &str) -> String {
        if let Some(host) = self.0 {
            format!("github:{owner}/{repo}/{rev}?host={host}")
        } else {
            format!("github:{owner}/{repo}/{rev}")
        }
    }
}
