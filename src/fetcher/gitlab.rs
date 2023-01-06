use once_cell::unsync::OnceCell;
use url::Url;

use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
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

impl<'a> SimpleFetcher<'a, 2> for FetchFromGitLab<'a> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitLab";

    fn host(&'a self) -> Option<&'a str> {
        self.host
    }

    fn group(&'a self) -> Option<&'a str> {
        self.group.get().copied()
    }

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 2]> {
        let mut xs = url.path_segments()?;
        let x = xs.next()?;
        let y = xs.next()?;
        Some(match xs.next() {
            None | Some("" | "-") => [x, y.strip_suffix(".git").unwrap_or(y)],
            Some(z) => {
                let _ = self.group.set(x);
                [y, z.strip_suffix(".git").unwrap_or(z)]
            }
        })
    }
}

impl<'a> SimpleFlakeFetcher<'a, 2> for FetchFromGitLab<'a> {
    fn get_flake_ref(&'a self, [owner, repo]: [&str; 2], rev: &str) -> String {
        let mut flake_ref = String::from("gitlab:");
        if let Some(group) = self.group.get() {
            flake_ref.push_str(group);
            flake_ref.push_str("%2F");
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
}
