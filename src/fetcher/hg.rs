use crate::{fetcher::UrlFlakeFetcher, impl_fetcher};

pub struct Fetchhg;
impl_fetcher!(Fetchhg);

impl UrlFlakeFetcher for Fetchhg {
    const FLAKE_TYPE: &'static str = "hg";
    const NAME: &'static str = "fetchhg";
}
