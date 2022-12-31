use crate::{
    fetcher::{SimpleFetcher, SimpleFlakeFetcher},
    impl_fetcher,
};

pub struct FetchFromSourcehut(pub Option<String>);
impl_fetcher!(FetchFromSourcehut);

impl<'a> SimpleFetcher<'a> for FetchFromSourcehut {
    const NAME: &'static str = "fetchFromSourcehut";

    fn host(&'a self) -> &'a Option<String> {
        &self.0
    }
}

impl<'a> SimpleFlakeFetcher<'a> for FetchFromSourcehut {
    const FLAKE_TYPE: &'static str = "sourcehut";
}
