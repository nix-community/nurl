use crate::{
    common::{CvsFetcher, CvsFlakeFetcher},
    impl_fetcher,
};

pub struct Fetchgit;
impl_fetcher!(Fetchgit);

impl CvsFetcher for Fetchgit {
    const NAME: &'static str = "fetchgit";
}

impl CvsFlakeFetcher for Fetchgit {
    const FLAKE_TYPE: &'static str = "git";
}
