use crate::{fetcher::SimpleFlakeFetcher, impl_fetcher};

pub struct FetchFromGitHub(pub Option<String>);
impl_fetcher!(FetchFromGitHub);

impl<'a> SimpleFlakeFetcher<'a> for FetchFromGitHub {
    const FLAKE_TYPE: &'static str = "github";
    const NAME: &'static str = "fetchFromGitHub";

    fn host(&'a self) -> &'a Option<String> {
        &self.0
    }
}
