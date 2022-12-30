use crate::fetcher::UrlFlakeFetcher;

pub struct Fetchgit;

impl UrlFlakeFetcher for Fetchgit {
    const NAME: &'static str = "fetchgit";
    const FLAKE_TYPE: &'static str = "git";
}
