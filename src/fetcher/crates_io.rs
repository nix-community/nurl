use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleUrlFetcher},
    Url,
};

pub struct FetchCrate(pub bool);
impl_fetcher!(FetchCrate);

impl SimpleFetcher<'_, 1> for FetchCrate {
    const KEYS: [&'static str; 1] = ["pname"];
    const NAME: &'static str = "fetchCrate";
    const REV_KEY: &'static str = "version";

    fn get_values<'a>(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        let mut xs = url.path_segments();
        Some([if self.0 {
            xs.nth(1)?
        } else {
            match xs.next()? {
                "crates" | "install" => xs.next()?,
                pname => pname,
            }
        }])
    }
}

impl SimpleUrlFetcher<'_, 1> for FetchCrate {
    fn get_url(&self, [pname]: &[&str; 1], version: &str) -> String {
        format!("https://crates.io/api/v1/crates/{pname}/{version}/download")
    }
}
