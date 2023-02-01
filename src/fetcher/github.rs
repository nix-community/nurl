use anyhow::{Context, Result};
use serde::Deserialize;

use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleGitFetcher},
};

pub struct FetchFromGitHub<'a>(pub Option<&'a str>);
impl_fetcher!(FetchFromGitHub<'a>);

#[derive(Deserialize)]
struct Commit {
    sha: String,
}

impl SimpleFetcher<'_, 2> for FetchFromGitHub<'_> {
    const HOST_KEY: &'static str = "githubBase";
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitHub";
    const SUBMODULES_KEY: Option<&'static str> = Some("fetchSubmodules");

    fn host(&self) -> Option<&str> {
        self.0
    }

    fn fetch_rev(&self, [owner, repo]: &[&str; 2]) -> Result<String> {
        let host = self.0.unwrap_or("github.com");
        let url = format!("https://api.{host}/repos/{owner}/{repo}/commits?per_page=1");

        let [Commit { sha }] = ureq::get(&url)
            .call()?
            .into_json::<[_; 1]>()
            .with_context(|| format!("no commits found for https://{host}/{owner}/{repo}"))?;

        Ok(sha)
    }
}

impl<'a> SimpleGitFetcher<'a, 2> for FetchFromGitHub<'a> {
    fn get_flake_ref(&self, [owner, repo]: &[&str; 2], rev: &str) -> String {
        if let Some(host) = self.0 {
            format!("github:{owner}/{repo}/{rev}?host={host}")
        } else {
            format!("github:{owner}/{repo}/{rev}")
        }
    }

    fn get_repo_url(&self, [owner, repo]: &[&str; 2]) -> String {
        format!(
            "git+https://{}/{owner}/{repo}",
            self.0.unwrap_or("github.com"),
        )
    }
}
