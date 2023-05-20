use assert_cmd::Command;
use nu_glob::glob;
use trycmd::TestCases;

use std::fs;

#[test]
fn integration() {
    TestCases::new()
        .default_bin_name("nurl")
        .case("tests/cmd/**/*.toml");
}

#[test]
fn verify_outputs() {
    for path in glob("tests/cmd/**/*.stdout").unwrap() {
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

            if name == "overwrite" {
                expr.insert_str(0, r#"let pname = "nurl"; in "#);
            } else if name == "overwrite_str" {
                expr.insert_str(0, r#"let version = "0.3.0"; in "#);
            }
        }

        Command::new("nix")
            .arg("build")
            .arg("--experimental-features")
            .arg("nix-command")
            .arg("--impure")
            .arg("--expr")
            .arg(expr)
            .unwrap();
    }
}
