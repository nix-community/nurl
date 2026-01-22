use crate::{
    Url, impl_fetcher,
    simple::{SimpleFetcher, SimpleFodFetcher},
};

pub struct Fetchsvn;
impl_fetcher!(Fetchsvn);

impl<'a> SimpleFetcher<'a, 1> for Fetchsvn {
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchsvn";

    fn get_values(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([url.as_str()])
    }
}

impl SimpleFodFetcher<'_, 1> for Fetchsvn {}
