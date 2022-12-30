use crate::fetcher::SimpleFlakeFetcher;

pub struct FetchFromSourcehut<'a>(pub Option<&'a str>);

impl<'a> SimpleFlakeFetcher<'a> for FetchFromSourcehut<'a> {
    const FLAKE_TYPE: &'static str = "sourcehut";
    const NAME: &'static str = "fetchFromSourcehut";

    fn host(&self) -> Option<&'a str> {
        self.0
    }
}
