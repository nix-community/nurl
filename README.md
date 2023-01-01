# nurl

[![release](https://img.shields.io/github/v/release/nix-community/nurl?logo=github&style=flat-square)](https://github.com/nix-community/nurl/releases)
[![version](https://img.shields.io/crates/v/nurl?logo=rust&style=flat-square)][crate]
[![deps](https://deps.rs/repo/github/nix-community/nurl/status.svg?style=flat-square&compact=true)](https://deps.rs/repo/github/nix-community/nurl)
[![license](https://img.shields.io/badge/license-MPL--2.0-blue?style=flat-square)](https://www.mozilla.org/en-US/MPL/2.0)
[![ci](https://img.shields.io/github/actions/workflow/status/nix-community/nurl/ci.yml?label=ci&logo=github-actions&style=flat-square)](https://github.com/nix-community/nurl/actions?query=workflow:ci)

Generate Nix fetcher calls from repository URLs

```console
$ nurl https://github.com/nix-community/patsh v0.2.0 2>/dev/null
fetchFromGitHub {
  owner = "nix-community";
  repo = "patsh";
  rev = "v0.2.0";
  hash = "sha256-7HXJspebluQeejKYmVA7sy/F3dtU1gc4eAbKiPexMMA=";
}
```

## Supported Fetchers

- fetchFromBitBucket
- fetchFromGitHub
- fetchFromGitLab
- fetchFromGitea
- fetchFromGitiles
- fetchFromRepoOrCz
- fetchFromSourcehut
- fetchgit
- fetchhg (requires `--fetcher fetchhg`)

## Usage

```
Usage: nurl [OPTIONS] [URL] [REV]

Arguments:
  [URL]  URL to the repository to be fetched
  [REV]  the revision or reference to be fetched

Options:
  -f, --fetcher <FETCHER>       specify the fetcher function instead of inferring from the URL [possible values: fetchFromBitBucket, fetchFromGitHub, fetchFromGitLab, fetchFromGitea, fetchFromGitiles, fetchFromRepoOrCz, fetchFromSourcehut, fetchgit, fetchhg]
  -i, --indent <INDENT>         extra indentation (in number of spaces) [default: 0]
  -j, --json                    output in json format
  -a, --arg <KEY> <VALUE>       additional arguments to pass to the fetcher
  -l, --list-fetchers           List all available fetchers
  -L, --list-possible-fetchers  List all fetchers that can be generated without --fetcher
  -h, --help                    Print help information
  -V, --version                 Print version information
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md)

[crate]: https://crates.io/crates/nurl
