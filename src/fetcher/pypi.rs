use anyhow::Result;

use crate::{
    Url,
    config::FetcherConfig,
    impl_fetcher,
    prefetch::url_prefetch,
    simple::{RevKey, SimpleFetcher},
};

pub struct FetchPypi;
impl_fetcher!(FetchPypi);

impl<'a> SimpleFetcher<'a, 1> for FetchPypi {
    const KEYS: [&'static str; 1] = ["pname"];
    const NAME: &'static str = "fetchPypi";
    const REV_KEY: RevKey = RevKey::Const("version");

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        let pname = url.path_segments().nth(1)?;
        (!pname.is_empty()).then_some([pname])
    }
}

impl FetchPypi {
    fn fetch(
        &self,
        values @ [pname]: &[&str; 1],
        rev_key: &'static str,
        version: &str,
        submodules: bool,
        cfg: &FetcherConfig,
    ) -> Result<String> {
        if !cfg.has_args() {
            url_prefetch(get_url(pname, version, "tar.gz"), false)
        } else if cfg.args.is_empty()
            && cfg.args_str.len() == 1
            && let Some(ext) = cfg.args.get("extension")
        {
            url_prefetch(get_url(pname, version, ext), false)
        } else {
            self.fetch_fod(values, rev_key, version, submodules, cfg)
        }
    }
}

fn get_url(pname: &str, version: &str, ext: &str) -> String {
    let Some(first) = pname.chars().next() else {
        unreachable!();
    };
    format!("https://pypi.org/packages/source/{first}/{pname}/{pname}-{version}.{ext}")
}
