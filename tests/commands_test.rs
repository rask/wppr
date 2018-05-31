extern crate wppr;

use std::env;
use wppr::commands::*;
use wppr::config::*;
use wppr::pipeline::*;
use wppr::wordpress::*;

#[path = "./testfns.rs"]
mod testfns;

fn get_test_plugin_index() -> String {
    testfns::get_tests_dir("data/plugins/test-plugin/plugin.php")
        .to_str()
        .unwrap()
        .to_string()
}

fn get_test_config() -> Config {
    let testcfg = Config {
        binaries: BinariesConfig {
            git: "/bin/true".to_string(),
            wpcli: "/bin/true".to_string(),
        },
        plugins: Some(vec![
            PluginConfig {
                index_path: get_test_plugin_index(),
                remote_repository: "/foo/bar.git".to_string(),
                package_name: "hello/world".to_string(),
            },
            PluginConfig {
                index_path: get_test_plugin_index(),
                remote_repository: "/qwe/asd/zxc.git".to_string(),
                package_name: "hello2/world2".to_string(),
            },
        ]),
        git: GitConfig {
            user_email: "".to_string(),
            user_name: "".to_string(),
            force_push: Some(false),
        },
        verbose: Some(false),
        dry_run: Some(false),
        cwd: Some("".to_string()),
    };

    testcfg
}

#[test]
fn test_all_plugins_can_be_fetched() {
    let mut plugins: Vec<Plugin> = get_managed_plugins(get_test_config());

    assert_eq!(plugins.len(), 2);

    let first: Plugin = plugins.remove(0);
    let second: Plugin = plugins.remove(0);

    assert_eq!(first.package_name, "hello/world");
    assert_eq!(second.package_name, "hello2/world2");
}

#[test]
fn test_plugin_pipelines_can_be_created() {
    let plugins = get_managed_plugins(get_test_config());

    for plugin in plugins {
        let pipeline = Pipeline::new(plugin);
    }
}
