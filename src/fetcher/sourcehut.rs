use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct FetchFromSourcehut(pub Option<String>);
impl_fetcher!(FetchFromSourcehut);

impl<'a> SimpleFetcher<'a> for FetchFromSourcehut {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromSourcehut";

    fn host(&'a self) -> Option<&'a str> {
        self.0.as_deref()
    }
}

impl<'a> SimpleFlakeFetcher<'a> for FetchFromSourcehut {
    const FLAKE_TYPE: &'static str = "sourcehut";
}
