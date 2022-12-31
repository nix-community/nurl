use crate::{
    common::{SimpleFetcher, SimpleFlakeFetcher},
    impl_fetcher,
};

pub struct FetchFromSourcehut(pub Option<String>);
impl_fetcher!(FetchFromSourcehut);

impl<'a> SimpleFetcher<'a> for FetchFromSourcehut {
    const NAME: &'static str = "fetchFromSourcehut";

    fn host(&'a self) -> Option<&'a str> {
        self.0.as_deref()
    }
}

impl<'a> SimpleFlakeFetcher<'a> for FetchFromSourcehut {
    const FLAKE_TYPE: &'static str = "sourcehut";
}
