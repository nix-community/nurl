use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
};

pub struct FetchFromRepoOrCz;
impl_fetcher!(FetchFromRepoOrCz);

impl SimpleFetcher<'_, 1> for FetchFromRepoOrCz {
    const KEYS: [&'static str; 1] = ["repo"];
    const NAME: &'static str = "fetchFromRepoOrCz";
}

impl SimpleUrlFetcher<'_, 1> for FetchFromRepoOrCz {
    fn get_url(&self, [repo]: &[&str; 1], rev: &str) -> String {
        format!("https://repo.or.cz/{repo}.git/snapshot/{rev}.tar.gz")
    }
}
