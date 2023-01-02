mod bitbucket;
mod git;
mod gitea;
mod github;
mod gitiles;
mod gitlab;
mod hg;
mod repo_or_cz;
mod sourcehut;

pub use bitbucket::FetchFromBitbucket;
pub use git::Fetchgit;
pub use gitea::FetchFromGitea;
pub use github::FetchFromGitHub;
pub use gitiles::FetchFromGitiles;
pub use gitlab::FetchFromGitLab;
pub use hg::Fetchhg;
pub use repo_or_cz::FetchFromRepoOrCz;
pub use sourcehut::FetchFromSourcehut;

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use rustc_hash::FxHashMap;
use url::Url;

use std::io::Write;

#[enum_dispatch]
pub trait Fetcher {
    fn fetch_nix(
        &self,
        out: &mut impl Write,
        url: &Url,
        rev: String,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
        overwrites: FxHashMap<String, String>,
        indent: String,
    ) -> Result<()>;
    fn fetch_json(
        &self,
        out: &mut impl Write,
        url: &Url,
        rev: String,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
        overwrites: Vec<(String, String)>,
        overwrites_str: Vec<(String, String)>,
    ) -> Result<()>;
}

#[enum_dispatch(Fetcher)]
pub enum FetcherDispatch<'a> {
    FetchFromBitbucket(FetchFromBitbucket),
    FetchFromGitHub(FetchFromGitHub<'a>),
    FetchFromGitLab(FetchFromGitLab<'a>),
    FetchFromGitea(FetchFromGitea<'a>),
    FetchFromGitiles(FetchFromGitiles),
    FetchFromRepoOrCz(FetchFromRepoOrCz),
    FetchFromSourcehut(FetchFromSourcehut<'a>),
    Fetchgit(Fetchgit),
    Fetchhg(Fetchhg),
}

#[macro_export]
macro_rules! impl_fetcher {
    ($t:ident $($tt:tt)*) => {
        impl $($tt)* $crate::fetcher::Fetcher for $t $($tt)* {
            fn fetch_nix(
                &self,
                out: &mut impl ::std::io::Write,
                url: &::url::Url,
                rev: String,
                args: Vec<(String, String)>,
                args_str: Vec<(String, String)>,
                overwrites: ::rustc_hash::FxHashMap<String, String>,
                indent: String,
            ) -> ::anyhow::Result<()> {
                self.fetch_nix_impl(out, url, rev, args, args_str, overwrites, indent)
            }

            fn fetch_json(
                &self,
                out: &mut impl ::std::io::Write,
                url: &::url::Url,
                rev: String,
                args: Vec<(String, String)>,
                args_str: Vec<(String, String)>,
                overwrites: Vec<(String, String)>,
                overwrites_str: Vec<(String, String)>,
            ) -> ::anyhow::Result<()> {
                self.fetch_json_impl(out, url, rev, args, args_str, overwrites, overwrites_str)
            }
        }
    };
}
