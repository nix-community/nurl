use clap::{Parser, ValueEnum};
use url::Url;

/// Generate Nix fetcher calls from repository URLs
/// https://github.com/nix-community/nurl
#[derive(Parser)]
#[command(version, verbatim_doc_comment)]
pub struct Opts {
    /// URL to the repository to be fetched
    #[arg(
        required_unless_present = "command",
        default_value = "x:", // placeholder value, will not be accessed
        hide_default_value = true
    )]
    pub url: Url,

    /// the revision or reference to be fetched
    #[arg(
        required_unless_present = "command",
        default_value_t,
        hide_default_value = true
    )]
    pub rev: String,

    /// specify the fetcher function instead of inferring from the URL
    #[arg(short, long)]
    pub fetcher: Option<FetcherFunction>,

    /// extra indentation (in number of spaces)
    #[arg(short, long, default_value_t = 0)]
    pub indent: usize,

    /// output in json format
    #[arg(short, long)]
    pub json: bool,

    // additional arguments to pass to the fetcher
    #[arg(short, long = "arg", num_args = 2, value_names = ["KEY", "VALUE"])]
    pub args: Vec<String>,

    /// List all available fetchers
    #[arg(short, long, group = "command")]
    pub list_fetchers: bool,

    /// List all fetchers that can be generated without --fetcher
    #[arg(short = 'L', long, group = "command")]
    pub list_possible_fetchers: bool,
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "camelCase")]
pub enum FetcherFunction {
    FetchFromBitBucket,
    FetchFromGitHub,
    FetchFromGitLab,
    FetchFromGitea,
    FetchFromSourcehut,
    Fetchgit,
    Fetchhg,
}
