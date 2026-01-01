use crate::{
    Url, impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
};

pub struct FetchHex;
impl_fetcher!(FetchHex);

impl<'a> SimpleFetcher<'a, 1> for FetchHex {
    const HASH_KEY: &'static str = "sha256";
    const KEYS: [&'static str; 1] = ["pkg"];
    const NAME: &'static str = "fetchHex";
    const REV_KEY: &'static str = "version";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([url.path_segments().nth(1)?])
    }
}

impl SimpleUrlFetcher<'_, 1> for FetchHex {
    const UNPACK: bool = false;

    fn get_url(&self, [pkg]: &[&str; 1], version: &str) -> String {
        format!("https://repo.hex.pm/tarballs/{pkg}-{version}.tar")
    }
}
