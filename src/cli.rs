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

    /// additional arguments to pass to the fetcher
    #[arg(short, long = "arg", num_args = 2, value_names = ["NAME", "EXPR"])]
    pub args: Vec<String>,

    /// same as --arg, but accepts strings instead Nix expressions
    #[arg(short = 'A', long = "arg-str", num_args = 2, value_names = ["NAME", "STRING"])]
    pub args_str: Vec<String>,

    /// overwrite arguments in the final output,
    /// not taken into consideration when fetching the hash
    ///
    /// Note that nurl does not verify any of the overwrites,
    /// for the final output to be valid,
    /// the user should not overwrite anything that would change the hash
    ///
    /// examples:
    /// {n}  --overwrite repo pname
    /// {n}  --overwrite rev version
    #[arg(short, long = "overwrite", num_args = 2, value_names = ["NAME", "EXPR"])]
    pub overwrites: Vec<String>,

    /// same as --overwrite, but accepts strings instead Nix expressions
    ///
    /// examples:
    /// {n}  --overwrite-str rev 'v${version}'
    /// {n}  --overwrite-str meta.homepage https://example.org
    #[arg(short = 'O', long = "overwrite-str", num_args = 2, value_names = ["NAME", "STRING"])]
    pub overwrites_str: Vec<String>,

    /// List all available fetchers
    #[arg(short, long, group = "command")]
    pub list_fetchers: bool,

    /// List all fetchers that can be generated without --fetcher
    #[arg(short = 'L', long, group = "command")]
    pub list_possible_fetchers: bool,

    /// Print out the listed fetchers with the specified separator,
    /// only used when --list-fetchers or --list-possible-fetchers is specified
    #[arg(
        short = 's',
        long,
        value_name = "SEPARATOR",
        allow_hyphen_values = true
    )]
    pub list_sep: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "camelCase")]
pub enum FetcherFunction {
    FetchFromBitbucket,
    FetchFromGitHub,
    FetchFromGitLab,
    FetchFromGitea,
    FetchFromGitiles,
    FetchFromRepoOrCz,
    FetchFromSourcehut,
    Fetchgit,
    Fetchhg,
    Fetchsvn,
}
