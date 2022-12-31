use crate::{
    common::{SimpleFetcher, SimpleFlakeFetcher},
    impl_fetcher,
};

pub struct FetchFromGitHub(pub Option<String>);
impl_fetcher!(FetchFromGitHub);

impl<'a> SimpleFetcher<'a> for FetchFromGitHub {
    const HOST_KEY: &'static str = "githubBase";
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromGitHub";

    fn host(&'a self) -> Option<&'a str> {
        self.0.as_deref()
    }
}

impl<'a> SimpleFlakeFetcher<'a> for FetchFromGitHub {
    const FLAKE_TYPE: &'static str = "github";
}
