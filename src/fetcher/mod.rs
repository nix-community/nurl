mod git;
mod gitea;
mod github;
mod gitlab;
mod hg;
mod sourcehut;

pub use git::Fetchgit;
pub use gitea::FetchFromGitea;
pub use github::FetchFromGitHub;
pub use gitlab::FetchFromGitLab;
pub use hg::Fetchhg;
pub use sourcehut::FetchFromSourcehut;

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use url::Url;

use std::io::Write;

#[enum_dispatch]
pub trait Fetcher {
    fn fetch_nix(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
        indent: String,
    ) -> Result<()>;
    fn fetch_json(
        &self,
        out: &mut impl Write,
        url: Url,
        rev: String,
        args: Vec<(String, String)>,
    ) -> Result<()>;
}

#[enum_dispatch(Fetcher)]
pub enum FetcherDispatch {
    FetchFromGitea(FetchFromGitea),
    FetchFromGitHub(FetchFromGitHub),
    FetchFromGitLab(FetchFromGitLab),
    FetchFromSourcehut(FetchFromSourcehut),
    Fetchgit(Fetchgit),
    Fetchhg(Fetchhg),
}

#[macro_export]
macro_rules! impl_fetcher {
    ($t:ty) => {
        impl $crate::fetcher::Fetcher for $t {
            fn fetch_nix(
                &self,
                out: &mut impl ::std::io::Write,
                url: ::url::Url,
                rev: String,
                args: Vec<(String, String)>,
                indent: String,
            ) -> ::anyhow::Result<()> {
                self.fetch_nix_impl(out, url, rev, args, indent)
            }

            fn fetch_json(
                &self,
                out: &mut impl ::std::io::Write,
                url: ::url::Url,
                rev: String,
                args: Vec<(String, String)>,
            ) -> ::anyhow::Result<()> {
                self.fetch_json_impl(out, url, rev, args)
            }
        }
    };
}
