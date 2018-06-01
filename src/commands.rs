//! # commands
//!
//! All command implementations.

use config::RuntimeConfig;
use pipeline::get_pipeline_for_plugin;
use prettytable::Table;
use wordpress::Plugin;

/// Get all plugins which are being managed.
pub fn get_managed_plugins(config: RuntimeConfig) -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = Vec::new();
    let cwd = config.cwd;

    let plugins_being_managed: Vec<_> = config.plugins;

    if plugins_being_managed.len() < 1 {
        return Vec::new();
    }

    for plugin_cfg in plugins_being_managed {
        plugins.push(Plugin::from_config(plugin_cfg, &cwd));
    }

    plugins
}

/// Lists managed WordPress plugins.
pub fn list(config: RuntimeConfig) -> Result<bool, &'static str> {
    println!("Listing managed plugins");

    let plugins: Vec<Plugin> = get_managed_plugins(config);

    if plugins.len() < 1 {
        println!("Configuration has no plugins defined");
        return Ok(true);
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
pub fn run(config: RuntimeConfig) -> Result<bool, &'static str> {
    let plugins: Vec<Plugin> = get_managed_plugins(config);
    let successes: Vec<bool> = Vec::new();
    let failures: Vec<&'static str> = Vec::new();

    if plugins.len() < 1 {
        println!("Configuration has no plugins defined");
        return Ok(true);
    }

    let mut plugin_table = Table::new();

    plugin_table.add_row(row!["Plugin", "Result", "Notes"]);

    for plugin in plugins {
        let valid = plugin.is_valid();
        let p_nicename = plugin.nicename.clone().unwrap_or("invalid".to_string());

        if !valid {
            plugin_table.add_row(row![
                &p_nicename,
                "false",
                "Plugin invalid, cannot run upgrades"
            ]);

            continue;
        }

        let pipeline = get_pipeline_for_plugin(plugin);

        let result = pipeline.run();

        match result {
            Ok(_) => {
                plugin_table.add_row(row![&p_nicename, "true", ""]);
            }
            Err(e) => {
                plugin_table.add_row(row![&p_nicename, "false", e]);
            }
        };
    }

    plugin_table.printstd();

    Ok(true)
}
