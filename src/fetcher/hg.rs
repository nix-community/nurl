use crate::fetcher::UrlFlakeFetcher;

pub struct Fetchhg;

impl UrlFlakeFetcher for Fetchhg {
    const FLAKE_TYPE: &'static str = "hg";
    const NAME: &'static str = "fetchhg";
}
