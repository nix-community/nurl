use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFlakeFetcher},
};

pub struct FetchFromGitLab(pub Option<String>);
impl_fetcher!(FetchFromGitLab);

impl<'a> SimpleFetcher<'a> for FetchFromGitLab {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitLab";

    fn host(&'a self) -> Option<&'a str> {
        self.0.as_deref()
    }
}

impl<'a> SimpleFlakeFetcher<'a> for FetchFromGitLab {
    const FLAKE_TYPE: &'static str = "gitlab";
}
