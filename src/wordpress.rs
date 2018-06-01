//! # wordpress
//!
//! This module contains WordPress specific functionalities.

use config::PluginConfig;
use regex::Regex;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

/// Representation of a WP plugin.
#[derive(Debug)]
pub struct Plugin {
    pub index_path: PathBuf,
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
    /// use std::path::PathBuf;
    ///
    /// let plugin = wordpress::Plugin {
    ///     index_path: PathBuf::from(""),
    ///     package_name: "".to_string(),
    ///     remote_repository: "".to_string(),
    ///     installed_version: Some("1.2.3".to_string()),
    ///     nicename: None
    /// };
    ///
    /// let failing_plugin = wordpress::Plugin {
    ///     index_path: PathBuf::from(""),
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

    pub fn from_config(plugin_config: PluginConfig, config_dir: &PathBuf) -> Plugin {
        let absolute_index_path = format!(
            "{}/{}", config_dir.to_str().unwrap(),
            plugin_config.index_path
        );

        let mut plugin: Plugin = Plugin {
            index_path: PathBuf::from(absolute_index_path),
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
    let path: PathBuf = plugin.index_path.to_owned();

    let mut nicenameparts: Vec<&str> = vec![
        path.parent().unwrap().file_name().unwrap().to_str().unwrap(),
        path.file_name().unwrap().to_str().unwrap()
    ];

    nicenameparts.join("/")
}

fn get_plugin_index_file_contents(index_path: &PathBuf) -> String {
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
            index_path: PathBuf::from("path/to/plugin/index.php"),
            remote_repository: "".to_string(),
            package_name: "".to_string(),
            installed_version: None,
            nicename: None,
        };

        let nicename: String = get_plugin_nicename(&plugin);

        assert_eq!(nicename, "plugin/index.php".to_string());
    }
}
