extern crate wppr;

use std::path::PathBuf;
use std::env;
use wppr::config;
use wppr::wordpress::{get_plugin_version, Plugin};

#[path = "./testfns.rs"]
mod testfns;

fn get_test_plugin_index() -> PathBuf {
    testfns::get_tests_dir("data/plugins/test-plugin/plugin.php")
}

#[test]
fn test_plugin_versions_can_be_read() {
    let plugin = Plugin {
        index_path: get_test_plugin_index(),
        package_name: "".to_string(),
        remote_repository: "".to_string(),
        installed_version: None,
        nicename: None,
        pre_cmds: Vec::new()
    };

    let version = get_plugin_version(&plugin).ok().unwrap();

    assert_eq!("0.1.2", version);
}

#[test]
fn test_plugin_can_be_created_from_config() {
    let pluginconfig = config::PluginConfig {
        index_path: get_test_plugin_index().to_str().unwrap().to_string(),
        package_name: "".to_string(),
        remote_repository: "".to_string(),
        pre_cmds: None
    };

    let plugin = Plugin::from_config(pluginconfig, &PathBuf::from(""));

    assert_eq!(plugin.index_path, get_test_plugin_index());
}
