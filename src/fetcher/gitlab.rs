use crate::fetcher::SimpleFlakeFetcher;

pub struct FetchFromGitLab<'a>(pub Option<&'a str>);

impl<'a> SimpleFlakeFetcher<'a> for FetchFromGitLab<'a> {
    const FLAKE_TYPE: &'static str = "gitlab";
    const NAME: &'static str = "fetchFromGitLab";

    fn host(&self) -> Option<&'a str> {
        self.0
    }
}
