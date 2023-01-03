use trycmd::TestCases;

#[test]
fn integration() {
    TestCases::new()
        .default_bin_name("nurl")
        .case("tests/cmd/**/*.toml");
}
