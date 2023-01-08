use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct FetchFromGitHub<'a>(pub Option<&'a str>);
impl_fetcher!(FetchFromGitHub<'a>);

#[derive(Deserialize)]
struct Commit {
    sha: String,
}

impl<'a> SimpleFetcher<'a, 2> for FetchFromGitHub<'a> {
    const HOST_KEY: &'static str = "githubBase";
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitHub";

    fn host(&'a self) -> Option<&'a str> {
        self.0
    }

    fn fetch_rev(&self, [owner, repo]: &[&str; 2]) -> Result<String> {
        let host = self.0.unwrap_or("github.com");

        Ok(
            ureq::get(&format!("https://api.{host}/repos/{owner}/{repo}/commits"))
                .call()?
                .into_json::<Vec<Commit>>()?
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("no commits found for https://{host}/{owner}/{repo}"))?
                .sha,
        )
    }
}

impl<'a> SimpleFlakeFetcher<'a, 2> for FetchFromGitHub<'a> {
    fn get_flake_ref(&'a self, [owner, repo]: &[&str; 2], rev: &str) -> String {
        if let Some(host) = self.0 {
            format!("github:{owner}/{repo}/{rev}?host={host}")
        } else {
            format!("github:{owner}/{repo}/{rev}")
        }
    }
}
