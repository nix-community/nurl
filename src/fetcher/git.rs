use crate::{
    fetcher::{UrlFetcher, UrlFlakeFetcher},
    impl_fetcher,
};

pub struct Fetchgit;
impl_fetcher!(Fetchgit);

impl UrlFetcher for Fetchgit {
    const NAME: &'static str = "fetchgit";
}

impl UrlFlakeFetcher for Fetchgit {
    const FLAKE_TYPE: &'static str = "git";
}
