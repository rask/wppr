extern crate assert_cli;
extern crate wppr;

use std::env;

fn get_cwd() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

#[test]
fn test_app_config_works() {
    let cfg_file: &str = &format!("{}/tests/data/libtestwppr.toml", get_cwd());
    let bin: &str = &format!("{}/target/debug/wppr", get_cwd());

    assert_cli::Assert::command(&[bin, "help"])
        .succeeds()
        .stdout()
        .contains("list")
        .unwrap();

    assert_cli::Assert::command(&[bin, "--configuration", cfg_file, "list"])
        .succeeds()
        .stdout()
        .contains("plugin.php")
        .stdout()
        .contains("0.1.2")
        .unwrap();

    assert_cli::Assert::command(&[bin, "list"])
        .fails()
        .unwrap();

    assert_cli::Assert::command(&[bin, "--configuration", "./relative/path.toml", "list"])
        .fails()
        .stderr()
        .contains("given as an absolute path")
        .unwrap();
}
