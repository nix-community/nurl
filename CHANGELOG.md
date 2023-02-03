# Changelog

## v0.3.7 - 2023-02-03

### Features

- `--submodules` to fetch submodules instead of using the fetcher's default
- `--nixpkgs` to override the nixpkgs used for fixed-output derivations
- Recognize `invent.kde.org` as a GitLab instance
- Recognize `repo.palemoon.org` as a Gitea instance

## v0.3.6 - 2023-01-24

### Features

- `--fallback` to specify a fetcher to fall back to
- `fetchPypi` support
- `builtins.fetchGit` support

## v0.3.5 - 2023-01-13

### Features

- `--hash` to only output the hash
- `--parse` to parse the URL into JSON without fetching the hash
- `fetchCrate` support
- `fetchHex` support

## v0.3.4 - 2023-01-11

### Fixes

- Correctly handle GitLab URLs that point to something other than the root (e.g. issues) of a project without a group

### Features

- Support fetching the latest revision for `fetchFromGitHub`, `fetchFromGitLab`, and `fetchFromGitea`
- Allow hyphen-separated values for `--lisp-sep`
- Recognize `salsa.debian.org` as a GitLab instance

## v0.3.3 - 2023-01-03

### Fixes

- Correctly handle trailing `.git` with `fetchFromGitLab`

## v0.3.2 - 2023-01-03

### Fixes

- Fix `fetchhg` output: `hash` -> `sha256`

### Features

- Support GitLab repositories under groups
- `--list-sep` to print out the listed fetchers with the specified separator
- `fetchsvn` support
- `fetchFromGitea` support for <https://gitea.com> and <https://notabug.org>
- Better support for URL schemes

### Internal

- Lots of tests

## v0.3.1 - 2023-01-02

### Fixes

- Correctly fetch with FOD when `--arg-str` is specified for URL fetchers

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
