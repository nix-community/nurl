use std::{cell::OnceCell, fmt::Write, iter::once};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::{
    Url, impl_fetcher,
    simple::{RevKey, SimpleFetcher, SimpleGitFetcher},
};

pub struct FetchFromGitLab<'a> {
    pub host: Option<&'a str>,
    pub group: OnceCell<&'a str>,
}
impl_fetcher!(FetchFromGitLab<'a>);

impl<'a> FetchFromGitLab<'a> {
    pub fn new(host: Option<&'a str>) -> Self {
        Self {
            host,
            group: OnceCell::new(),
        }
    }
}

#[derive(Deserialize)]
struct Commit {
    id: String,
}

impl<'a> SimpleFetcher<'a, 2> for FetchFromGitLab<'a> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitLab";
    const REV_KEY: RevKey = RevKey::RevOrTag;
    const SUBMODULES_KEY: Option<&'static str> = Some("fetchSubmodules");

    fn host(&self) -> Option<&str> {
        self.host
    }

    fn group(&self) -> Option<&str> {
        self.group.get().copied()
    }

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 2]> {
        let mut i = 0;
        for j in url
            .path
            .match_indices('/')
            .map(|(j, _)| j)
            .chain(once(url.path.len()))
        {
            if matches!(url.path.get(i + 1 .. j), Some("" | "-") | None) {
                break;
            }
            i = j;
        }

        let (path, repo) = url.path[.. i].rsplit_once('/')?;
        let owner = match path.rsplit_once('/') {
            Some((group, owner)) => {
                let _ = self.group.set(group);
                owner
            }
            None => path,
        };

        Some([owner, repo.strip_suffix(".git").unwrap_or(repo)])
    }

    fn fetch_rev(&self, [owner, repo]: &[&str; 2]) -> Result<String> {
        let host = self.host.unwrap_or("gitlab.com");

        let mut url = format!("https://{host}/api/v4/projects/");
        if let Some(group) = self.group.get() {
            url.push_str(&group.replace('/', "%2F"));
            url.push_str("%2F");
        }
        write!(url, "{owner}%2F{repo}/repository/commits?per_page=1")?;

        let [Commit { id }] = ureq::get(&url)
            .call()?
            .into_body()
            .read_json::<[_; 1]>()
            .with_context(|| {
                let mut msg = format!("no commits found for https://{host}/");
                if let Some(group) = self.group.get() {
                    msg.push_str(group);
                    msg.push('/');
                }
                msg.push_str(owner);
                msg.push('/');
                msg.push_str(repo);
                msg
            })?;

        Ok(id)
    }
}

impl<'a> SimpleGitFetcher<'a, 2> for FetchFromGitLab<'a> {
    fn get_flake_ref(&self, [owner, repo]: &[&str; 2], rev: &str) -> String {
        let mut flake_ref = String::from("gitlab:");
        if let Some(group) = self.group.get() {
            flake_ref.push_str(&group.replace('/', "%252F"));
            flake_ref.push_str("%252F");
        }
        flake_ref.push_str(owner);
        flake_ref.push('/');
        flake_ref.push_str(repo);
        flake_ref.push('/');
        flake_ref.push_str(rev);
        if let Some(host) = self.host {
            flake_ref.push_str("?host=");
            flake_ref.push_str(host);
        }
        flake_ref
    }

    fn get_repo_url(&self, [owner, repo]: &[&str; 2]) -> String {
        let mut flake_ref = String::from("git+https://");
        flake_ref.push_str(self.host.unwrap_or("gitlab.com"));
        flake_ref.push('/');
        if let Some(group) = self.group.get() {
            flake_ref.push_str(group);
            flake_ref.push('/');
        }
        flake_ref.push_str(owner);
        flake_ref.push('/');
        flake_ref.push_str(repo);
        flake_ref
    }
}

#[cfg(test)]
mod tests {
    use super::FetchFromGitLab;
    use crate::{Url, simple::SimpleFetcher};

    #[test]
    fn basic() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/foo/bar",
            path: "foo/bar",
        };
        assert_eq!(fetcher.get_values(&url), Some(["foo", "bar"]));
        assert_eq!(fetcher.group(), None);
    }

    #[test]
    fn basic_issues() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/foo/bar/-/issues/42",
            path: "foo/bar/-/issues/42",
        };
        assert_eq!(fetcher.get_values(&url), Some(["foo", "bar"]));
        assert_eq!(fetcher.group(), None);
    }

    #[test]
    fn group() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/foo/bar/baz",
            path: "foo/bar/baz",
        };
        assert_eq!(fetcher.get_values(&url), Some(["bar", "baz"]));
        assert_eq!(fetcher.group(), Some("foo"));
    }

    #[test]
    fn nested() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/lorem/ipsum/dolor/sit/amet",
            path: "lorem/ipsum/dolor/sit/amet",
        };
        assert_eq!(fetcher.get_values(&url), Some(["sit", "amet"]));
        assert_eq!(fetcher.group(), Some("lorem/ipsum/dolor"));
    }

    #[test]
    fn nested_issues() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/lorem/ipsum/dolor/sit/amet/-/issues/42",
            path: "lorem/ipsum/dolor/sit/amet/-/issues/42",
        };
        assert_eq!(fetcher.get_values(&url), Some(["sit", "amet"]));
        assert_eq!(fetcher.group(), Some("lorem/ipsum/dolor"));
    }

    #[test]
    fn nested_trailing() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/lorem/ipsum/dolor/sit/amet//",
            path: "lorem/ipsum/dolor/sit/amet//",
        };
        assert_eq!(fetcher.get_values(&url), Some(["sit", "amet"]));
        assert_eq!(fetcher.group(), Some("lorem/ipsum/dolor"));
    }

    #[test]
    fn invalid() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/lorem",
            path: "lorem",
        };
        assert_eq!(fetcher.get_values(&url), None);
    }

    #[test]
    fn invalid_trailing() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/lorem/",
            path: "lorem/",
        };
        assert_eq!(fetcher.get_values(&url), None);
    }

    #[test]
    fn invalid_double_trailing() {
        let fetcher = FetchFromGitLab::new(None);
        let url = Url {
            url: "https://gitlab.com/lorem//",
            path: "lorem//",
        };
        assert_eq!(fetcher.get_values(&url), None);
    }
}
