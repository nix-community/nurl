# Changelog

## v0.3.0 - 2023-01-02

### Changes

- with `--json`, values specified by `--arg` are wrapped like this: `{"type": "nix", "value": "<Nix expression>"}` to differentiate from strings

### Fixes

- Correctly handle git:// URLs

### Features

- `--arg-str` to additional arguments to pass to the fetcher as strings
- `--overwrite` to overwrite arguments in the final output
- `--overwrite-str` to overwrite arguments in the final output as strings

## v0.2.2 - 2023-01-01

### Fixes

- Correctly set `experimental-features` for Nix calls
- Fetch submodules for `fetchgit`

## v0.2.1 - 2023-01-01

### Fixes

- Typos

## v0.2.0 - 2023-01-01

### Fixes

- `fetchFromGitHub` now correctly sets `githubBase` when domain is not github.com

### Features

- `--arg` to pass extra rguments to the fetcher
- `--json` to output in json format
- `--list-fetchers` to list all available fetchers
- `--list-possible-fetchers` to list all fetchers that can be generated without `--fetcher`
- Support for the following fetchers
  - `fetchFromBitbucket` (<https://bitbucket.org>)
  - `fetchFromGitea` (and <https://codeberg.org>)
  - `fetchFromGitiles` (and <https://googlesource.com>)
  - `fetchFromRepoOrCz` (<https://repo.or.cz>)
- Man page
- Shell completions
- Nix flake
- Colored output

## v0.1.1 - 2022-12-29

First release
