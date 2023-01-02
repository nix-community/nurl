use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct FetchFromGitLab<'a>(pub Option<&'a str>);
impl_fetcher!(FetchFromGitLab<'a>);

impl<'a> SimpleFetcher<'a, 2> for FetchFromGitLab<'a> {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitLab";

    fn host(&'a self) -> Option<&'a str> {
        self.0
    }
}

impl<'a> SimpleFlakeFetcher<'a, 2> for FetchFromGitLab<'a> {
    const FLAKE_TYPE: &'static str = "gitlab";
}
