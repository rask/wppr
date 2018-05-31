extern crate assert_cli;
extern crate wppr;

use std::env;
use std::path::PathBuf;

#[path = "./testfns.rs"]
mod testfns;

#[test]
fn test_app_config_works() {
    let cfg_file: PathBuf = testfns::get_tests_dir("data/libtestwppr.toml");
    let mut binpath: PathBuf = testfns::get_cwd();

    binpath.push("target/debug/wppr");

    let bin = binpath.to_str().unwrap();

    assert_cli::Assert::command(&[bin, "help"])
        .succeeds()
        .stdout()
        .contains("list")
        .unwrap();

    assert_cli::Assert::command(&[bin, "--configuration", cfg_file.to_str().unwrap(), "list"])
        .succeeds()
        .stdout()
        .contains("plugin.php")
        .stdout()
        .contains("0.1.2")
        .unwrap();

    assert_cli::Assert::command(&[bin, "list"]).fails().unwrap();

    assert_cli::Assert::command(&[bin, "--configuration", "./relative/path.toml", "list"])
        .fails()
        .stderr()
        .contains("given as an absolute path")
        .unwrap();
}
