use crate::{
    common::{SimpleFetcher, SimpleFlakeFetcher},
    impl_fetcher,
};

pub struct FetchFromGitHub(pub Option<String>);
impl_fetcher!(FetchFromGitHub);

impl<'a> SimpleFetcher<'a> for FetchFromGitHub {
    const NAME: &'static str = "fetchFromGitHub";

    fn host(&'a self) -> &'a Option<String> {
        &self.0
    }
}

impl<'a> SimpleFlakeFetcher<'a> for FetchFromGitHub {
    const FLAKE_TYPE: &'static str = "github";
}
