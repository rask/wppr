extern crate wppr;

use std::env;
use wppr::wordpress::{get_plugin_version, Plugin, PluginFromConfig};
use wppr::config;

fn get_cwd() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

fn get_test_plugin_index() -> String {
    let cwd: String = get_cwd();

    let index_path = cwd + "/tests/data/plugins/test-plugin/plugin.php";

    index_path
}

#[test]
fn test_plugin_versions_can_be_read() {
    let plugin = Plugin {
        index_path: get_test_plugin_index(),
        package_name: "".to_string(),
        remote_repository: "".to_string(),
        installed_version: None,
        nicename: None,
    };

    let version = get_plugin_version(&plugin);

    assert_eq!("0.1.2", version);
}

#[test]
fn test_plugin_can_be_created_from_config() {
    let pluginconfig = config::PluginConfig {
        index_path: get_test_plugin_index(),
        package_name: "".to_string(),
        remote_repository: "".to_string(),
    };

    let plugin = Plugin::from_config(pluginconfig, "");

    assert_eq!(plugin.index_path, format!("/{}", get_test_plugin_index()));
}
