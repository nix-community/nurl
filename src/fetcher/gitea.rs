use anyhow::{Context, Result};
use serde::Deserialize;

use crate::{
    impl_fetcher,
    prefetch::{git_prefetch, url_prefetch},
    simple::SimpleFetcher,
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
    const SUBMODULES_KEY: Option<&'static str> = Some("fetchSubmodules");

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
            .into_body()
            .read_json::<[_; 1]>()
            .with_context(|| format!("no commits found for https://{}/{owner}/{repo}", self.0))?;

        Ok(sha)
    }
}

impl FetchFromGitea<'_> {
    fn fetch(
        &self,
        values @ [owner, repo]: &[&str; 2],
        rev: &str,
        submodules: bool,
        args: &[(String, String)],
        args_str: &[(String, String)],
        nixpkgs: String,
    ) -> Result<String> {
        if args.is_empty() && args_str.is_empty() {
            if submodules {
                git_prefetch(
                    true,
                    &format!("git+https://{}/{owner}/{repo}", self.0),
                    rev,
                    true,
                )
            } else {
                url_prefetch(
                    format!("https://{}/{owner}/{repo}/archive/{rev}.tar.gz", self.0),
                    true,
                )
            }
        } else {
            self.fetch_fod(values, rev, submodules, args, args_str, nixpkgs)
        }
    }
}
