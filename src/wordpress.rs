//! # wordpress
//!
//! This module contains WordPress specific functionalities.

use config::PluginConfig;
use std::fs::File;
use std::io::Read;
use regex::Regex;

/// Representation of a WP plugin.
pub struct Plugin {
    pub index_path: String,
    pub package_name: String,
    pub remote_repository: String,
    pub installed_version: Option<String>,
    pub nicename: Option<String>,
}

impl Plugin {
    /// Checks if a plugin is valid.
    ///
    /// # Example
    ///
    /// ```
    /// use wppr::wordpress;
    ///
    /// let plugin = wordpress::Plugin {
    ///     index_path: "".to_string(),
    ///     package_name: "".to_string(),
    ///     remote_repository: "".to_string(),
    ///     installed_version: Some("1.2.3".to_string()),
    ///     nicename: None
    /// };
    ///
    /// let failing_plugin = wordpress::Plugin {
    ///     index_path: "".to_string(),
    ///     package_name: "".to_string(),
    ///     remote_repository: "".to_string(),
    ///     installed_version: None,
    ///     nicename: None
    /// };
    ///
    /// assert!(plugin.is_valid());
    /// assert!(!failing_plugin.is_valid());
    pub fn is_valid(&self) -> bool {
        let valid_version = match self.installed_version {
            Some(_) => true,
            None => false,
        };

        valid_version
    }
}

/// Load plugin struct data from config data.
pub trait PluginFromConfig {
    fn from_config(plugin_config: PluginConfig, config_dir: &str) -> Plugin;
}

/// Allows creating a plugin instance from a config.
impl PluginFromConfig for Plugin {
    fn from_config(plugin_config: PluginConfig, config_dir: &str) -> Plugin {
        let absolute_index_path = format!("{}/{}", config_dir, plugin_config.index_path);

        let mut plugin: Plugin = Plugin {
            index_path: absolute_index_path,
            package_name: plugin_config.package_name,
            remote_repository: plugin_config.remote_repository,
            installed_version: None,
            nicename: None,
        };

        let nicename = get_plugin_nicename(&plugin);
        let installed_version = get_plugin_version(&plugin);

        plugin.nicename = Some(nicename.to_owned());
        plugin.installed_version = Some(installed_version.to_owned());

        plugin
    }
}

/// Get the WP convention dir/file.php nicename for a plugin.
fn get_plugin_nicename(plugin: &Plugin) -> String {
    let path: String = plugin.index_path.to_owned();
    let mut parts: Vec<&str> = path.split("/").collect();

    let mut nicenameparts: Vec<&str> = vec![parts.pop().unwrap(), parts.pop().unwrap()];

    nicenameparts.reverse();
    nicenameparts.join("/")
}

fn get_plugin_index_file_contents(index_path: &str) -> String {
    let mut contents: String = String::new();

    File::open(index_path)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    contents
}

pub fn get_plugin_version(plugin: &Plugin) -> String {
    let index_contents: String = get_plugin_index_file_contents(&plugin.index_path);

    let version_matcher = Regex::new(r"Version:\s+(\d+\.\d+\.\d+)\s+").unwrap();

    version_matcher
        .captures(&index_contents)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_nicename_can_be_fetched() {
        let plugin = Plugin {
            index_path: "path/to/plugin/index.php".to_string(),
            remote_repository: "".to_string(),
            package_name: "".to_string(),
            installed_version: None,
            nicename: None,
        };

        let nicename: String = get_plugin_nicename(&plugin);

        assert_eq!(nicename, "plugin/index.php".to_string());
    }
}
