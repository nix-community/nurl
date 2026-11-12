use std::io::{IsTerminal, stdout};

use clap::Parser;
use eyre::Result;
use nurl::cli::Opts;
use supports_color::Stream;

fn main() -> Result<()> {
    if supports_color::on(Stream::Stderr).is_some() {
        color_eyre::install()?;
    }
    let opts = Opts::parse();
    let terminal = stdout().is_terminal();
    nurl::nurl_impl(opts, &mut stdout().lock(), terminal)
}
