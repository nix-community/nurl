use crate::{fetcher::SimpleFlakeFetcher, impl_fetcher};

pub struct FetchFromSourcehut(pub Option<String>);
impl_fetcher!(FetchFromSourcehut);

impl<'a> SimpleFlakeFetcher<'a> for FetchFromSourcehut {
    const FLAKE_TYPE: &'static str = "sourcehut";
    const NAME: &'static str = "fetchFromSourcehut";

    fn host(&'a self) -> &'a Option<String> {
        &self.0
    }
}
