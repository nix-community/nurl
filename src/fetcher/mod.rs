mod bitbucket;
mod builtin_git;
mod crates_io;
mod git;
mod gitea;
mod github;
mod gitiles;
mod gitlab;
mod hex;
mod hg;
mod pypi;
mod repo_or_cz;
mod sourcehut;
mod svn;

use std::io::Write;

use anyhow::Result;
use enum_dispatch::enum_dispatch;

pub use self::{
    bitbucket::FetchFromBitbucket, builtin_git::BuiltinsFetchGit, crates_io::FetchCrate,
    git::Fetchgit, gitea::FetchFromGitea, github::FetchFromGitHub, gitiles::FetchFromGitiles,
    gitlab::FetchFromGitLab, hex::FetchHex, hg::Fetchhg, pypi::FetchPypi,
    repo_or_cz::FetchFromRepoOrCz, sourcehut::FetchFromSourcehut, svn::Fetchsvn,
};
use crate::{Url, config::FetcherConfig};

#[enum_dispatch]
pub trait Fetcher<'a> {
    fn fetch_nix(&self, out: &mut impl Write, url: &'a Url, cfg: FetcherConfig) -> Result<()>;

    fn fetch_hash(&self, out: &mut impl Write, url: &'a Url, cfg: FetcherConfig) -> Result<()>;

    fn fetch_json(&self, out: &mut impl Write, url: &'a Url, cfg: FetcherConfig) -> Result<()>;

    fn to_json(&'a self, out: &mut impl Write, url: &'a Url, rev: Option<String>) -> Result<()>;
}

#[enum_dispatch(Fetcher)]
pub enum FetcherDispatch<'a> {
    BuiltinsFetchGit(BuiltinsFetchGit),
    FetchCrate(FetchCrate),
    FetchFromBitbucket(FetchFromBitbucket),
    FetchFromGitHub(FetchFromGitHub<'a>),
    FetchFromGitLab(FetchFromGitLab<'a>),
    FetchFromGitea(FetchFromGitea<'a>),
    FetchFromGitiles(FetchFromGitiles),
    FetchFromRepoOrCz(FetchFromRepoOrCz),
    FetchFromSourcehut(FetchFromSourcehut<'a>),
    FetchHex(FetchHex),
    FetchPypi(FetchPypi),
    Fetchgit(Fetchgit),
    Fetchhg(Fetchhg),
    Fetchsvn(Fetchsvn),
}

#[macro_export]
macro_rules! impl_fetcher {
    ($t:ty) => {
        impl<'a> $crate::fetcher::Fetcher<'a> for $t {
            fn fetch_nix(
                &self,
                out: &mut impl ::std::io::Write,
                url: &'a $crate::Url,
                cfg: $crate::config::FetcherConfig,
            ) -> ::anyhow::Result<()> {
                use ::anyhow::Context;

                let values = &self
                    .get_values(url)
                    .with_context(|| format!("failed to parse {url}"))?;

                let rev = match &cfg.rev {
                    Some(rev) => rev.clone(),
                    None => self.fetch_rev(values)?,
                };
                let (rev_key, rev) = self.rev_entry(&rev);

                let submodules = self.resolve_submodules(cfg.submodules);
                let hash = self.fetch(values, rev_key, rev, submodules, &cfg)?;

                self.write_nix(out, values, rev_key, rev, hash, submodules, cfg)
            }

            fn fetch_hash(
                &self,
                out: &mut impl ::std::io::Write,
                url: &'a $crate::Url,
                cfg: $crate::config::FetcherConfig,
            ) -> ::anyhow::Result<()> {
                use ::anyhow::Context;

                let values = &self
                    .get_values(url)
                    .with_context(|| format!("failed to parse {url}"))?;

                let rev = match &cfg.rev {
                    Some(rev) => rev,
                    None => &self.fetch_rev(values)?,
                };
                let (rev_key, rev) = self.rev_entry(&rev);

                let submodules = self.resolve_submodules(cfg.submodules);
                let hash = self.fetch(values, rev_key, rev, submodules, &cfg)?;
                write!(out, "{hash}")?;

                Ok(())
            }

            fn fetch_json(
                &self,
                out: &mut impl ::std::io::Write,
                url: &'a $crate::Url,
                cfg: $crate::config::FetcherConfig,
            ) -> ::anyhow::Result<()> {
                use ::anyhow::Context;

                let values = &self
                    .get_values(url)
                    .with_context(|| format!("failed to parse {url}"))?;

                let rev = match &cfg.rev {
                    Some(rev) => rev.clone(),
                    None => self.fetch_rev(values)?,
                };
                let (rev_key, rev) = self.rev_entry(&rev);

                let submodules = self.resolve_submodules(cfg.submodules);
                let hash = self.fetch(values, rev_key, rev, submodules, &cfg)?;

                self.write_json(
                    out,
                    values,
                    rev_key,
                    rev,
                    hash,
                    submodules,
                    cfg,
                )
            }

            fn to_json(
                &'a self,
                out: &mut impl ::std::io::Write,
                url: &'a $crate::Url,
                rev: Option<String>,
            ) -> ::anyhow::Result<()> {
                use ::anyhow::Context;
                use ::serde_json::{json, Value};

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
                    let (rev_key, rev) = self.rev_entry(&rev);
                    fetcher_args[rev_key] = json!(rev);
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
