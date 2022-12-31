use crate::{
    common::{SimpleFetcher, SimpleUrlFetcher},
    impl_fetcher,
};

pub struct FetchFromRepoOrCz;
impl_fetcher!(FetchFromRepoOrCz);

impl<'a> SimpleFetcher<'a, 1> for FetchFromRepoOrCz {
    const KEYS: [&'static str; 1] = ["repo"];
    const NAME: &'static str = "fetchFromRepoOrCz";
}

impl<'a> SimpleUrlFetcher<'a, 1> for FetchFromRepoOrCz {
    fn get_url(&self, [repo]: [&str; 1], rev: &str) -> String {
        format!("https://repo.or.cz/{repo}.git/snapshot/{rev}.tar.gz")
    }
}
