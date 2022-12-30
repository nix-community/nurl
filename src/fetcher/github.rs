use crate::fetcher::SimpleFlakeFetcher;

pub struct FetchFromGitHub<'a>(pub Option<&'a str>);

impl<'a> SimpleFlakeFetcher<'a> for FetchFromGitHub<'a> {
    const FLAKE_TYPE: &'static str = "github";
    const NAME: &'static str = "fetchFromGitHub";

    fn host(&self) -> Option<&'a str> {
        self.0
    }
}
