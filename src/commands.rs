//! # commands
//!
//! All command implementations.

use config::Config;
use wordpress::{Plugin, PluginFromConfig};
use prettytable::Table;

/// Lists managed WordPress plugins.
pub fn list(config: Config) -> Result<bool, &'static str> {
    println!("Listing managed plugins");

    let mut plugins: Vec<Plugin> = Vec::new();
    let cwd = config.cwd.unwrap();

    let plugins_being_managed = match config.plugins {
        Some(p) => p,
        None => {
            println!("Configuration has no plugins defined");
            return Ok(true);
        }
    };

    for plugin_cfg in plugins_being_managed {
        plugins.push(Plugin::from_config(plugin_cfg, &cwd));
    }

    let mut plugin_table = Table::new();

    plugin_table.add_row(row!["Plugin", "Valid", "Version", "Package name", "Remote"]);

    for plugin in plugins {
        let validity = match plugin.is_valid() {
            true => "true",
            false => "false",
        };

        plugin_table.add_row(row![
            &plugin.nicename.unwrap_or("invalid".to_string()),
            &validity,
            &plugin.installed_version.unwrap_or("unknown".to_string()),
            &plugin.package_name,
            &plugin.remote_repository
        ]);
    }

    plugin_table.printstd();

    Ok(true)
}

/// Runs upgrades and gitifications on managed WordPress plugins.
pub fn run(config: Config) -> Result<bool, &'static str> {
    Err("Not implemented")
}
