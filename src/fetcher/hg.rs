use crate::{
    common::{CvsFetcher, CvsFlakeFetcher},
    impl_fetcher,
};

pub struct Fetchhg;
impl_fetcher!(Fetchhg);

impl CvsFetcher for Fetchhg {
    const NAME: &'static str = "fetchhg";
}

impl CvsFlakeFetcher for Fetchhg {
    const FLAKE_TYPE: &'static str = "hg";
}
