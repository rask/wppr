//! # wordpress
//!
//! This module contains WordPress specific functionalities.

use config::PluginConfig;
use regex::Regex;
use std::process::Command;
use std::path::{PathBuf, Component};
use std::fs::File;
use std::io::Read;

/// Representation of a WP plugin.
#[derive(Debug, Clone)]
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

    /// Get the nicename of this plugin, e.g. `dir/index.php`.
    pub fn get_nicename(&self) -> String {
        match self.nicename {
            Some(ref s) => s.clone(),
            None => "invalid".to_string()
        }
    }

    /// Get a WpCli usable plugin dir name.
    pub fn get_cli_name(&self) -> Result<String, String> {
        let nicename = self.get_nicename();

        if nicename == "invalid" {
            return Err("invalid".to_string());
        }

        let nicepath = PathBuf::from(nicename);
        let mut niceiter = nicepath.iter();
        let dir = niceiter.nth(0).unwrap();

        return Ok(dir.to_os_string().into_string().unwrap());
    }

    pub fn from_config(plugin_config: PluginConfig, config_dir: &PathBuf) -> Plugin {
        let absolute_index_path = format!(
            "{}/{}",
            config_dir.to_str().unwrap(),
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

    /// Get a .git directory which is inside the plugin directory.
    pub fn get_git_dir(&self) -> Result<PathBuf, String> {
        let mut gitdir = self.index_path.clone();
        gitdir.pop();
        gitdir.push(".git");

        if gitdir.exists() && gitdir.is_dir() {
            return Ok(gitdir);
        }

        return Err(format!(
            "Cannot fetch git directory for plugin `{}`, not a directory",
            self.nicename.clone().unwrap_or("invalid".to_string())
        ));
    }

    /// Get a composer.json which is inside the plugin directory.
    pub fn get_composerjson_file(&self) -> Result<PathBuf, String> {
        let mut cjson = self.index_path.clone();
        cjson.pop();
        cjson.push("composer.json");

        if cjson.exists() {
            return Ok(cjson);
        }

        return Err(format!(
            "Cannot fetch composer.json for plugin `{}`, does not exist",
            self.nicename.clone().unwrap_or("invalid".to_string())
        ));
    }
}

/// Get the WP convention dir/file.php nicename for a plugin.
fn get_plugin_nicename(plugin: &Plugin) -> String {
    let path: PathBuf = plugin.index_path.to_owned();

    let mut nicenameparts: Vec<&str> = vec![
        path.parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        path.file_name().unwrap().to_str().unwrap(),
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

/// WpCli wrapper.
pub struct WpCli {
    bin: String,
    working_directory: PathBuf
}

pub type WpCliResult = Result<bool, String>;

impl WpCli {
    /// Get a new WpCli wrapper instance.
    pub fn new(bin: String, cwd: PathBuf) -> Self {
        WpCli {
            bin: bin,
            working_directory: cwd
        }
    }

    /// Get a base command to use in other commands.
    fn get_base_command(&self) -> Command {
        let bin = self.bin.clone();
        let cwd = self.working_directory.clone();

        let mut cmd = Command::new(bin);

        cmd.current_dir(cwd);

        return cmd;
    }

    /// Update a single WP plugin with WpCli.
    pub fn update_plugin(&self, plugin: &Plugin) -> WpCliResult {
        let pname = plugin.get_cli_name()?;

        let mut cmd = self.get_base_command();

        cmd.args(&["plugin", "update", &pname]);

        let output = cmd.output().expect("Error when trying to run wp-cli command: `wp update plugin ...`");

        match output.status.success() {
            true => Ok(true),
            false => {
                Err(format!("Could not update plugin `{}`: `{}`", pname, String::from_utf8_lossy(&output.stderr)))
            }
        }
    }
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
