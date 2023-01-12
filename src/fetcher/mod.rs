mod bitbucket;
mod git;
mod gitea;
mod github;
mod gitiles;
mod gitlab;
mod hg;
mod repo_or_cz;
mod sourcehut;
mod svn;

pub use bitbucket::FetchFromBitbucket;
pub use git::Fetchgit;
pub use gitea::FetchFromGitea;
pub use github::FetchFromGitHub;
pub use gitiles::FetchFromGitiles;
pub use gitlab::FetchFromGitLab;
pub use hg::Fetchhg;
pub use repo_or_cz::FetchFromRepoOrCz;
pub use sourcehut::FetchFromSourcehut;
pub use svn::Fetchsvn;

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use rustc_hash::FxHashMap;
use url::Url;

use std::io::Write;

#[enum_dispatch]
pub trait Fetcher<'a> {
    fn fetch_nix(
        &'a self,
        out: &mut impl Write,
        url: &'a Url,
        rev: Option<String>,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
        overwrites: FxHashMap<String, String>,
        indent: String,
    ) -> Result<()>;

    fn fetch_hash(
        &'a self,
        out: &mut impl ::std::io::Write,
        url: &'a Url,
        rev: Option<String>,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
    ) -> Result<()>;

    fn fetch_json(
        &'a self,
        out: &mut impl Write,
        url: &'a Url,
        rev: Option<String>,
        args: Vec<(String, String)>,
        args_str: Vec<(String, String)>,
        overwrites: Vec<(String, String)>,
        overwrites_str: Vec<(String, String)>,
    ) -> Result<()>;

    fn to_json(
        &'a self,
        out: &mut impl ::std::io::Write,
        url: &'a ::url::Url,
        rev: Option<String>,
    ) -> ::anyhow::Result<()>;
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
    Fetchsvn(Fetchsvn),
}

#[macro_export]
macro_rules! impl_fetcher {
    ($t:ty) => {
        impl<'a> $crate::fetcher::Fetcher<'a> for $t {
            fn fetch_nix(
                &'a self,
                out: &mut impl ::std::io::Write,
                url: &'a ::url::Url,
                rev: Option<String>,
                args: Vec<(String, String)>,
                args_str: Vec<(String, String)>,
                overwrites: ::rustc_hash::FxHashMap<String, String>,
                indent: String,
            ) -> ::anyhow::Result<()> {
                use anyhow::Context;

                let values = &self
                    .get_values(url)
                    .with_context(|| format!("failed to parse {url}"))?;

                let rev = match rev {
                    Some(rev) => rev,
                    None => self.fetch_rev(values)?,
                };

                let hash = self.fetch(values, &rev, &args, &args_str)?;

                self.write_nix(out, values, rev, hash, args, args_str, overwrites, indent)
            }

            fn fetch_hash(
                &'a self,
                out: &mut impl ::std::io::Write,
                url: &'a ::url::Url,
                rev: Option<String>,
                args: Vec<(String, String)>,
                args_str: Vec<(String, String)>,
            ) -> ::anyhow::Result<()> {
                use anyhow::Context;

                let values = &self
                    .get_values(url)
                    .with_context(|| format!("failed to parse {url}"))?;

                let rev = match rev {
                    Some(rev) => rev,
                    None => self.fetch_rev(values)?,
                };

                let hash = self.fetch(values, &rev, &args, &args_str)?;
                write!(out, "{}", hash)?;

                Ok(())
            }

            fn fetch_json(
                &'a self,
                out: &mut impl ::std::io::Write,
                url: &'a ::url::Url,
                rev: Option<String>,
                args: Vec<(String, String)>,
                args_str: Vec<(String, String)>,
                overwrites: Vec<(String, String)>,
                overwrites_str: Vec<(String, String)>,
            ) -> ::anyhow::Result<()> {
                use anyhow::Context;

                let values = &self
                    .get_values(url)
                    .with_context(|| format!("failed to parse {url}"))?;

                let rev = match rev {
                    Some(rev) => rev,
                    None => self.fetch_rev(values)?,
                };

                let hash = self.fetch(values, &rev, &args, &args_str)?;

                self.write_json(
                    out,
                    values,
                    rev,
                    hash,
                    args,
                    args_str,
                    overwrites,
                    overwrites_str,
                )
            }

            fn to_json(
                &'a self,
                out: &mut impl ::std::io::Write,
                url: &'a ::url::Url,
                rev: Option<String>,
            ) -> ::anyhow::Result<()> {
                use anyhow::Context;
                use serde_json::{json, Value};

                let values = self
                    .get_values(url)
                    .with_context(|| format!("failed to parse {url}"))?;

                let mut fetcher_args = Value::from_iter(Self::KEYS.into_iter().zip(values));

                if let Some(host) = self.host() {
                    fetcher_args[Self::HOST_KEY] = json!(host);
                }
                if let Some(group) = self.group() {
                    fetcher_args["group"] = json!(group);
                }
                if let Some(rev) = rev {
                    fetcher_args["rev"] = json!(rev);
                }

                serde_json::to_writer(
                    out,
                    &json!({
                        "fetcher": Self::NAME,
                        "args": fetcher_args,
                    }),
                )?;

                Ok(())
            }
        }
    };
}
