//! config_test.rs
//! 
//! Integration tests for configurations.

extern crate wppr;
extern crate toml;

#[path="./testfns.rs"]
mod testfns;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use wppr::config::*;

#[test]
fn test_configuration_is_loaded_from_toml() {
    let src_toml = testfns::get_tests_dir("data/valid.toml");

    let mut cfg_data: String = String::new();

    File::open(src_toml)
        .unwrap()
        .read_to_string(&mut cfg_data)
        .unwrap();

    let mut configuration: TomlConfig = toml::from_str(&cfg_data).unwrap();

    assert_eq!(configuration.binaries.unwrap().git, "/usr/bin/my-git".to_string());
    assert_eq!(configuration.plugins.unwrap()[0].package_name, "test/package".to_string());
}

#[test]
fn test_configuration_is_flattened_to_runtime_config() {
    let tomlcfg: TomlConfig = TomlConfig {
        binaries: Some(BinariesConfig {
            git: "/bin/true".to_string(),
            wpcli: "/bin/true".to_string()
        }),
        git: Some(GitConfig {
            user_name: "wppr".to_string(),
            user_email: "wppe@wppr.wppr".to_string(),
            force_push: false
        }),
        plugins: Some(vec![
            PluginConfig {
                package_name: "hello/world-package".to_string(),
                index_path: "pkgs/hello/world.php".to_string(),
                remote_repository: "../hello-world.git".to_string()
            },
            PluginConfig {
                package_name: "foo/bar-package".to_string(),
                index_path: "pkgs/foo/bar.php".to_string(),
                remote_repository: "../bar.git".to_string()
            }
        ]),
        verbose: Some(false),
        dry_run: Some(true),
        cwd: Some("/my/cwd/path".to_string())
    };

    let runtimecfg = RuntimeConfig::from_toml_config(tomlcfg).unwrap();

    assert_eq!(runtimecfg.cwd, PathBuf::from("/my/cwd/path"));
    assert_eq!(runtimecfg.plugins[1].package_name, "foo/bar-package".to_string());
}