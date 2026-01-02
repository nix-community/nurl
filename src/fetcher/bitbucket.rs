use crate::{
    impl_fetcher,
    simple::{RevKey, SimpleFetcher, SimpleUrlFetcher},
};

pub struct FetchFromBitbucket;
impl_fetcher!(FetchFromBitbucket);

impl SimpleFetcher<'_, 2> for FetchFromBitbucket {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromBitbucket";
    const REV_KEY: RevKey = RevKey::RevOrTag;
}

impl SimpleUrlFetcher<'_, 2> for FetchFromBitbucket {
    fn get_url(&self, [owner, repo]: &[&str; 2], rev: &str) -> String {
        format!("https://bitbucket.org/{owner}/{repo}/get/{rev}.tar.gz")
    }
}
