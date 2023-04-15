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

- builtins.fetchGit
- fetchCrate
- fetchFromBitbucket
- fetchFromGitHub
- fetchFromGitLab
- fetchFromGitea
- fetchFromGitiles
- fetchFromRepoOrCz
- fetchFromSourcehut
- fetchHex
- fetchPypi
- fetchgit
- fetchhg
- fetchsvn

## Usage

```
Usage: nurl [OPTIONS] [URL] [REV]

Arguments:
  [URL]  URL to the repository to be fetched
  [REV]  The revision or reference to be fetched

Options:
  -S, --submodules[=<SUBMODULES>]      Fetch submodules instead of using the fetcher's default [possible
                                       values: true, false]
  -f, --fetcher <FETCHER>              Specify the fetcher function instead of inferring from the
                                       URL [possible values: builtins.fetchGit, fetchCrate,
                                       fetchFromBitbucket, fetchFromGitHub, fetchFromGitLab,
                                       fetchFromGitea, fetchFromGitiles, fetchFromRepoOrCz,
                                       fetchFromSourcehut, fetchHex, fetchPypi, fetchgit, fetchhg,
                                       fetchsvn]
  -F, --fallback <FALLBACK>            The fetcher to fall back to when nurl fails to infer it from
                                       the URL [default: fetchgit] [possible values:
                                       builtins.fetchGit, fetchCrate, fetchFromBitbucket,
                                       fetchFromGitHub, fetchFromGitLab, fetchFromGitea,
                                       fetchFromGitiles, fetchFromRepoOrCz, fetchFromSourcehut,
                                       fetchHex, fetchPypi, fetchgit, fetchhg, fetchsvn]
  -n, --nixpkgs <NIXPKGS>              Path to nixpkgs (in nix) [default: <nixpkgs>]
  -i, --indent <INDENT>                Extra indentation (in number of spaces) [default: 0]
  -H, --hash                           Only output the hash
  -j, --json                           Output in json format
  -p, --parse                          Parse the url without fetching the hash, output in json
                                       format
  -a, --arg <NAME> <EXPR>              Additional arguments to pass to the fetcher
  -A, --arg-str <NAME> <STRING>        Same as --arg, but accepts strings instead Nix expressions
  -o, --overwrite <NAME> <EXPR>        Overwrite arguments in the final output, not taken into
                                       consideration when fetching the hash
  -O, --overwrite-str <NAME> <STRING>  Same as --overwrite, but accepts strings instead Nix
                                       expressions
  -e, --expr <EXPR>                    Instead of fetching a URL, get the hash of a fixed-output
                                       derivation, implies --hash and ignores all other options
  -l, --list-fetchers                  List all available fetchers
  -L, --list-possible-fetchers         List all fetchers that can be generated without --fetcher
  -s, --list-sep <SEPARATOR>           Print out the listed fetchers with the specified separator,
                                       only used when --list-fetchers or --list-possible-fetchers is
                                       specified
  -h, --help                           Print help
  -V, --version                        Print version
```

## Comparison to [nix-prefetch](https://github.com/msteen/nix-prefetch)

- `nurl` infers the fetcher from the URL. For `nix-prefetch`, you need to pick the fetcher and supply the arguments manually.
- `nix-prefetch` relies on FOD which is slow, `nurl` tries to use alternatives when possible.
- `nix-prefetch` is more configurable and supports file attributes.
- `nix-prefetch` has an interface similar to `nix-build`.
- `nurl` has some nice features dedicated to generated packages (`--indent`, `--list-possible-fetchers`).

## Changelog

See [CHANGELOG.md](CHANGELOG.md)

[crate]: https://crates.io/crates/nurl
