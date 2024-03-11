use std::{
    io::BufRead,
    process::{Command, Output, Stdio},
};

use anyhow::{anyhow, bail, Result};
use data_encoding::BASE64;
use nix_compat::nixbase32;
use serde::Deserialize;

trait GetStdout {
    fn get_stdout(&mut self) -> Result<Vec<u8>>;
}

impl GetStdout for Command {
    fn get_stdout(&mut self) -> Result<Vec<u8>> {
        let Output { stdout, status, .. } = self.stderr(Stdio::inherit()).output()?;
        if !status.success() {
            bail!("command exited with {}", status);
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

    info!("$ nix flake prefetch --extra-experimental-features 'nix-command flakes' --json {flake_ref}");
    Ok(serde_json::from_slice::<PrefetchOutput>(
        &Command::new("nix")
            .arg("flake")
            .arg("prefetch")
            .arg("--extra-experimental-features")
            .arg("nix-command flakes")
            .arg("--json")
            .arg(flake_ref)
            .get_stdout()?,
    )?
    .hash)
}

// work around for https://github.com/NixOS/nix/issues/5291
pub fn git_prefetch(git_scheme: bool, url: &str, rev: &str, submodules: bool) -> Result<String> {
    let prefix = if git_scheme { "" } else { "git+" };
    let submodules = if submodules { "&submodules=1" } else { "" };

    if rev.len() == 40 {
        flake_prefetch(format!("{prefix}{url}?allRefs=1&rev={rev}{submodules}"))
    } else {
        if !rev.starts_with("refs/") {
            if let hash @ Ok(_) =
                flake_prefetch(format!("{prefix}{url}?ref=refs/tags/{rev}{submodules}"))
            {
                return hash;
            }
        }
        flake_prefetch(format!("{prefix}{url}?ref={rev}{submodules}"))
    }
}

pub fn url_prefetch(url: String, unpack: bool) -> Result<String> {
    use bstr::ByteSlice;

    let mut cmd = Command::new("nix-prefetch-url");
    if unpack {
        cmd.arg("--unpack");
        info!("$ nix-prefetch-url --unpack {url}");
    } else {
        info!("$ nix-prefetch-url {url}");
    }
    cmd.arg(url);

    let hash = cmd.get_stdout()?;
    Ok(format!(
        "sha256-{}",
        BASE64.encode(&nixbase32::decode(hash.trim_end())?),
    ))
}

pub fn fod_prefetch(expr: String) -> Result<String> {
    info!(
        "$ nix build --extra-experimental-features 'nix-command flakes' --impure --no-link --expr '{expr}'"
    );

    let Output {
        stdout,
        stderr,
        status,
    } = Command::new("nix")
        .arg("build")
        .arg("--extra-experimental-features")
        .arg("nix-command flakes")
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
        let Some(line) = lines.next() else {
            break;
        };
        if let Ok(line) = line {
            let Some(hash) = line.trim_start().strip_prefix("got:") else {
                continue;
            };
            return Ok(hash.trim().into());
        }
    }

    Err(anyhow!(
        "failed to find the hash from error messages\nstdout: {}\nstderr:\n{}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr),
    ))
}
