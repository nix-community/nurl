use anyhow::{Context, Result};
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

impl SimpleFetcher<'_, 2> for FetchFromGitea<'_> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitea";

    fn host(&self) -> Option<&str> {
        Some(self.0)
    }

    fn fetch_rev(&self, [owner, repo]: &[&str; 2]) -> Result<String> {
        let url = format!(
            "https://{}/api/v1/repos/{owner}/{repo}/commits?limit=1&stat=false",
            self.0,
        );

        let [Commit { sha }] = ureq::get(&url)
            .call()?
            .into_json::<[_; 1]>()
            .with_context(|| format!("no commits found for https://{}/{owner}/{repo}", self.0))?;

        Ok(sha)
    }
}

impl<'a> SimpleUrlFetcher<'a, 2> for FetchFromGitea<'a> {
    fn get_url(&self, [owner, repo]: &[&str; 2], rev: &str) -> String {
        format!("https://{}/{owner}/{repo}/archive/{rev}.tar.gz", self.0)
    }
}
