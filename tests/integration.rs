use std::fs;

use assert_cmd::Command;
use nu_glob::{Uninterruptible, glob};
use trycmd::TestCases;

#[test]
fn integration() {
    TestCases::new()
        .default_bin_name("nurl")
        .case("tests/cmd/**/*.toml")
        // fetchFromGitLab fails for repositories within groups on GNOME GitLab
        // https://github.com/NixOS/nixpkgs/issues/485701
        .skip("tests/cmd/fetcher/gitlab/group.toml")
        // fetchFromRepoOrCz is flaky
        .skip("tests/cmd/fetcher/repo_or_cz.toml");
}

#[test]
fn verify_outputs() {
    for path in glob("tests/cmd/**/*.stdout", Uninterruptible).unwrap() {
        let path = path.unwrap();
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".stdout")
            .unwrap();

        if matches!(name, "expr" | "hash" | "json" | "parse") {
            eprintln!("skipping {}", path.display());
            continue;
        }

        eprintln!("testing {}", path.display());

        let mut expr = String::from_utf8(fs::read(&path).unwrap()).unwrap();

        if name != "builtin_git" {
            expr.insert_str(0, "(import <nixpkgs> { }).");

            if path.parent().unwrap().file_name().unwrap() == "overwrite" {
                if name == "basic" {
                    expr.insert_str(0, r#"let pname = "nurl"; in "#);
                } else {
                    expr.insert_str(0, r#"let version = "0.3.0"; in "#);
                }
            }
        }

        Command::new("nix")
            .arg("build")
            .arg("--extra-experimental-features")
            .arg("nix-command")
            .arg("--impure")
            .arg("--expr")
            .arg(expr)
            .unwrap();
    }
}
