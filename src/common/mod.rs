mod cvs;
mod simple;

pub use cvs::{CvsFetcher, CvsFlakeFetcher, CvsFodFetcher};
pub use simple::{SimpleFetcher, SimpleFlakeFetcher, SimpleFodFetcher};

use anyhow::{anyhow, bail, Result};
use serde::Deserialize;

use std::{
    io::BufRead,
    process::{Command, Output, Stdio},
};

trait GetStdout {
    fn get_stdout(&mut self) -> Result<Vec<u8>>;
}

impl GetStdout for Command {
    fn get_stdout(&mut self) -> Result<Vec<u8>> {
        let Output { stdout, status, .. } = self.stderr(Stdio::inherit()).output()?;
        if !status.success() {
            bail!("command exited with exit code {}", status);
        }
        Ok(stdout)
    }
}

pub fn flake_prefetch(flake_ref: String) -> Result<String> {
    #[derive(Deserialize)]
    struct PrefetchOutput {
        hash: String,
    }

    eprintln!("$ nix flake prefetch --json {flake_ref}");
    Ok(serde_json::from_slice::<PrefetchOutput>(
        &Command::new("nix")
            .arg("flake")
            .arg("prefetch")
            .arg("--json")
            .arg(flake_ref)
            .get_stdout()?,
    )?
    .hash)
}

pub fn fod_prefetch(expr: String) -> Result<String> {
    eprintln!("$ nix build --impure --no-link --expr '{expr}'");

    let Output {
        stdout,
        stderr,
        status,
    } = Command::new("nix")
        .arg("build")
        .arg("--impure")
        .arg("--no-link")
        .arg("--expr")
        .arg(expr)
        .output()?;

    if status.success() {
        bail!(
            "command succeeded unexpectedly\nstdout:\n{}",
            String::from_utf8_lossy(&stdout),
        );
    }

    let mut lines = stderr.lines();
    while let Some(line) = lines.next() {
        if !matches!(line, Ok(line) if line.trim_start().starts_with("specified:")) {
            continue;
        }
        let Some(line) = lines.next() else { break; };
        if let Ok(line) = line {
            let Some(hash) = line.trim_start().strip_prefix("got:") else { continue; };
            return Ok(hash.trim().into());
        }
    }

    Err(anyhow!(
        "failed to find the hash from error messages\nstdout: {}\nstderr:\n{}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr),
    ))
}
