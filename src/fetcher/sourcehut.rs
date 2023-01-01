use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct FetchFromSourcehut<'a>(pub Option<&'a str>);
impl_fetcher!(FetchFromSourcehut<'a>);

impl<'a> SimpleFetcher<'a> for FetchFromSourcehut<'a> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromSourcehut";

    fn host(&'a self) -> Option<&'a str> {
        self.0
    }
}

impl<'a> SimpleFlakeFetcher<'a> for FetchFromSourcehut<'a> {
    const FLAKE_TYPE: &'static str = "sourcehut";
}
