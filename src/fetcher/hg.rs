use crate::{
    fetcher::{UrlFetcher, UrlFlakeFetcher},
    impl_fetcher,
};

pub struct Fetchhg;
impl_fetcher!(Fetchhg);

impl UrlFetcher for Fetchhg {
    const NAME: &'static str = "fetchhg";
}

impl UrlFlakeFetcher for Fetchhg {
    const FLAKE_TYPE: &'static str = "hg";
}
