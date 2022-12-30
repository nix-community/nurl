use crate::{fetcher::UrlFlakeFetcher, impl_fetcher};

pub struct Fetchgit;
impl_fetcher!(Fetchgit);

impl UrlFlakeFetcher for Fetchgit {
    const FLAKE_TYPE: &'static str = "git";
    const NAME: &'static str = "fetchgit";
}
