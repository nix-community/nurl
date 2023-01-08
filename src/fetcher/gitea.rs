use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
};

pub struct FetchFromGitea<'a>(pub &'a str);
impl_fetcher!(FetchFromGitea<'a>);

#[derive(Deserialize)]
struct Commit {
    sha: String,
}

impl<'a> SimpleFetcher<'a, 2> for FetchFromGitea<'a> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitea";

    fn host(&'a self) -> Option<&'a str> {
        Some(self.0)
    }

    fn fetch_rev(&self, [owner, repo]: &[&str; 2]) -> Result<String> {
        let url = format!("https://{}/api/v1/repos/{owner}/{repo}/commits", self.0);
        Ok(ureq::get(&url)
            .call()?
            .into_json::<Vec<Commit>>()?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("no commits found for https://{}/{owner}/{repo}", self.0))?
            .sha)
    }
}

impl<'a> SimpleUrlFetcher<'a, 2> for FetchFromGitea<'a> {
    fn get_url(&self, [owner, repo]: &[&str; 2], rev: &str) -> String {
        format!("https://{}/{owner}/{repo}/archive/{rev}.tar.gz", self.0)
    }
}
