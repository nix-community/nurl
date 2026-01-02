use crate::{
    impl_fetcher,
    simple::{RevKey, SimpleFetcher, SimpleGitFetcher},
};

pub struct FetchFromSourcehut<'a>(pub Option<&'a str>);
impl_fetcher!(FetchFromSourcehut<'a>);

impl<'a> SimpleFetcher<'a, 2> for FetchFromSourcehut<'a> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromSourcehut";
    const REV_KEY: RevKey = RevKey::RevOrTag;
    const SUBMODULES_KEY: Option<&'static str> = Some("fetchSubmodules");

    fn host(&self) -> Option<&str> {
        self.0
    }
}

impl<'a> SimpleGitFetcher<'a, 2> for FetchFromSourcehut<'a> {
    fn get_flake_ref(&self, [owner, repo]: &[&str; 2], rev: &str) -> String {
        if let Some(host) = self.0 {
            format!("sourcehut:{owner}/{repo}/{rev}?host={host}")
        } else {
            format!("sourcehut:{owner}/{repo}/{rev}")
        }
    }

    fn get_repo_url(&self, [owner, repo]: &[&str; 2]) -> String {
        format!(
            "git+https://{}/{owner}/{repo}",
            self.0.unwrap_or("git.sr.ht"),
        )
    }
}
