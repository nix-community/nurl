use crate::{fetcher::SimpleFlakeFetcher, impl_fetcher};

pub struct FetchFromGitLab(pub Option<String>);
impl_fetcher!(FetchFromGitLab);

impl<'a> SimpleFlakeFetcher<'a> for FetchFromGitLab {
    const FLAKE_TYPE: &'static str = "gitlab";
    const NAME: &'static str = "fetchFromGitLab";

    fn host(&'a self) -> &'a Option<String> {
        &self.0
    }
}
