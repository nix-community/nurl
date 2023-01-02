use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct FetchFromGitLab<'a>(pub Option<&'a str>);
impl_fetcher!(FetchFromGitLab<'a>);

impl<'a> SimpleFetcher<'a, 2> for FetchFromGitLab<'a> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitLab";

    fn host(&'a self) -> Option<&'a str> {
        self.0
    }
}

impl<'a> SimpleFlakeFetcher<'a, 2> for FetchFromGitLab<'a> {
    fn get_flake_ref(&'a self, [owner, repo]: [&str; 2], rev: &str) -> String {
        if let Some(host) = self.0 {
            format!("gitlab:{owner}/{repo}/{rev}?host={host}")
        } else {
            format!("gitlab:{owner}/{repo}/{rev}")
        }
    }
}
