use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
};

pub struct FetchFromBitBucket;
impl_fetcher!(FetchFromBitBucket);

impl<'a> SimpleFetcher<'a> for FetchFromBitBucket {
    const KEYS: [&'static str; 2] = ["owner", "repo"];
    const NAME: &'static str = "fetchFromBitBucket";
}

impl<'a> SimpleUrlFetcher<'a> for FetchFromBitBucket {
    fn get_url(&self, [owner, repo]: [&str; 2], rev: &str) -> String {
        format!("https://bitbucket.org/{owner}/{repo}/get/{rev}.tar.gz")
    }
}
