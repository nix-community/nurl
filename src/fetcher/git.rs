use crate::fetcher::UrlFlakeFetcher;

pub struct Fetchgit;

impl UrlFlakeFetcher for Fetchgit {
    const FLAKE_TYPE: &'static str = "git";
    const NAME: &'static str = "fetchgit";
}
