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

macro_rules! info {
    ($($tt:tt)+) => {{
        use owo_colors::{OwoColorize, Stream, Style};
        eprintln!(
            "{}",
            format_args!($($tt)+).if_supports_color(Stream::Stderr, |text| text
                .style(Style::new().blue().bold()))
        );
    }};
}

pub fn flake_prefetch(flake_ref: String) -> Result<String> {
    #[derive(Deserialize)]
    struct PrefetchOutput {
        hash: String,
    }

    info!("$ nix flake prefetch --experimental-features 'nix-command flakes' --json {flake_ref}");
    Ok(serde_json::from_slice::<PrefetchOutput>(
        &Command::new("nix")
            .arg("flake")
            .arg("prefetch")
            .arg("--experimental-features")
            .arg("nix-command flakes")
            .arg("--json")
            .arg(flake_ref)
            .get_stdout()?,
    )?
    .hash)
}

pub fn url_prefetch(url: String, unpack: bool) -> Result<String> {
    let mut cmd = Command::new("nix-prefetch-url");
    if unpack {
        cmd.arg("--unpack");
        info!("$ nix-prefetch-url --unpack {url}");
    } else {
        info!("$ nix-prefetch-url {url}");
    }
    cmd.arg(url);

    let hash = String::from_utf8(cmd.get_stdout()?)?;
    let hash = hash.trim_end();

    info!("$ nix hash to-sri --experimental-features nix-command --type sha256 {hash}");
    Ok(String::from_utf8(
        Command::new("nix")
            .arg("hash")
            .arg("to-sri")
            .arg("--experimental-features")
            .arg("nix-command")
            .arg("--type")
            .arg("sha256")
            .arg(hash)
            .get_stdout()?,
    )?
    .trim_end()
    .into())
}

pub fn fod_prefetch(expr: String) -> Result<String> {
    info!("$ nix build --experimental-features nix-command --impure --no-link --expr '{expr}'");

    let Output {
        stdout,
        stderr,
        status,
    } = Command::new("nix")
        .arg("build")
        .arg("--experimental-features")
        .arg("nix-command")
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
