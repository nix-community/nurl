use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
    Url,
};

pub struct FetchPypi;
impl_fetcher!(FetchPypi);

impl<'a> SimpleFetcher<'a, 1> for FetchPypi {
    const KEYS: [&'static str; 1] = ["pname"];
    const NAME: &'static str = "fetchPypi";
    const REV_KEY: &'static str = "version";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        let pname = url.path_segments().nth(1)?;
        (!pname.is_empty()).then_some([pname])
    }
}

impl SimpleUrlFetcher<'_, 1> for FetchPypi {
    const UNPACK: bool = false;

    fn get_url(&self, [pname]: &[&str; 1], version: &str) -> String {
        let Some(first) = pname.chars().next() else { unreachable!(); };
        format!("https://pypi.org/packages/source/{first}/{pname}/{pname}-{version}.tar.gz")
    }
}
