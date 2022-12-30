use clap::{Parser, ValueEnum};
use url::Url;

/// Generate Nix fetcher calls from repository URLs
/// https://github.com/nix-community/nurl
#[derive(Parser)]
#[command(version, verbatim_doc_comment)]
pub struct Opts {
    /// URL to the repository to be fetched
    pub url: Url,

    /// the revision or reference to be fetched
    pub rev: String,

    /// specify the fetcher function instead of inferring from the URL
    #[arg(short, long)]
    pub fetcher: Option<FetcherFunction>,

    /// extra indentation (in number of spaces)
    #[arg(short, long, default_value_t = 0)]
    pub indent: usize,
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "camelCase")]
pub enum FetcherFunction {
    FetchFromGitHub,
    FetchFromGitLab,
    FetchFromSourcehut,
    Fetchgit,
    Fetchhg,
}
