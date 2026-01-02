use anyhow::Result;

use crate::{Url, impl_fetcher, prefetch::flake_prefetch, simple::SimpleFetcher};

pub struct Fetchhg(pub bool);
impl_fetcher!(Fetchhg);

impl<'a> SimpleFetcher<'a, 1> for Fetchhg {
    const HASH_KEY: &'static str = "sha256";
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchhg";
    const SUBMODULES_KEY: Option<&'static str> = Some("fetchSubrepos");

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([if self.0 {
            url.as_str().strip_prefix("hg+")?
        } else {
            url.as_str()
        }])
    }
}

impl Fetchhg {
    fn fetch(
        &self,
        values @ [url]: &[&str; 1],
        rev_key: &'static str,
        rev: &str,
        submodules: bool,
        args: &[(String, String)],
        args_str: &[(String, String)],
        nixpkgs: String,
    ) -> Result<String> {
        if args.is_empty() && args_str.is_empty() && !submodules {
            flake_prefetch(format!(
                "hg+{url}?{}={rev}",
                if rev.len() == 40 { "rev" } else { "ref" },
            ))
        } else {
            self.fetch_fod(values, rev_key, rev, submodules, args, args_str, nixpkgs)
        }
    }
}
