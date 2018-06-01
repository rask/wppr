//! # config
//!
//! Configuration related functionalities.

use std::path::PathBuf;
use std::process::{Command, Stdio};

//LCOV_EXCL_START
/// Struct to hold deserialized TOML configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct TomlConfig {
    pub binaries: Option<BinariesConfig>,
    pub git: Option<GitConfig>,
    pub plugins: Option<Vec<PluginConfig>>,
    pub verbose: Option<bool>,
    pub dry_run: Option<bool>,
    pub cwd: Option<String>,
}

/// Configuration of binaries used when running the tool.
#[derive(Debug, Deserialize, Clone)]
pub struct BinariesConfig {
    pub git: String,
    pub wpcli: String,
}

/// Configuration for Git when running the tool.
#[derive(Debug, Deserialize, Clone)]
pub struct GitConfig {
    pub user_name: String,
    pub user_email: String,
    pub force_push: bool,
}

/// Plugins configuration when running the tool.
#[derive(Debug, Deserialize, Clone)]
pub struct PluginConfig {
    pub index_path: String,
    pub package_name: String,
    pub remote_repository: String,
}
//LCOV_EXCL_STOP

impl TomlConfig {
    pub fn set_verbosity(&mut self, verbosity: bool) {
        self.verbose = Some(verbosity);
    }

    pub fn set_dry_run(&mut self, dry_run: bool) {
        self.dry_run = Some(dry_run);
    }

    pub fn set_cwd(&mut self, cwd: String) {
        self.cwd = Some(cwd);
    }
}

/// Validate configuration for the tool.
pub fn validate_configuration(config: &TomlConfig) -> Result<bool, &'static str> {
    let bins: BinariesConfig = config.clone().binaries.unwrap();
    let git_is_valid = validate_binary(&bins.git);
    let wp_cli_is_valid = validate_binary(&bins.wpcli);

    if !git_is_valid {
        return Err("Invalid git binary provided");
    } else if !wp_cli_is_valid {
        return Err("Invalid wp cli binary provided");
    }

    Ok(true)
}

/// Validate if a binary exists and that the binary returns a version.
fn validate_binary(bin: &String) -> bool {
    match Command::new(bin)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("--version")
        .spawn()
    {
        Ok(_) => true,
        Err(_e) => false,
    }
}

/// Runtime config that has been flattened from TomlConfig.
#[derive(Debug)]
pub struct RuntimeConfig {
    pub binaries: BinariesConfig,
    pub git: GitConfig,
    pub plugins: Vec<PluginConfig>,
    pub verbose: bool,
    pub dry_run: bool,
    pub cwd: PathBuf,
}

impl RuntimeConfig {
    pub fn from_toml_config(toml_config: TomlConfig) -> Result<RuntimeConfig, &'static str> {
        validate_configuration(&toml_config)?;

        Ok(RuntimeConfig {
            binaries: toml_config.binaries.unwrap(),
            git: toml_config.git.unwrap(),
            plugins: toml_config.plugins.unwrap_or(Vec::new()),
            verbose: toml_config.verbose.unwrap_or(false),
            dry_run: toml_config.dry_run.unwrap_or(false),
            cwd: PathBuf::from(toml_config.cwd.unwrap())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> TomlConfig {
        let testcfg = TomlConfig {
            binaries: Some(BinariesConfig {
                git: "/bin/true".to_string(),
                wpcli: "/bin/true".to_string(),
            }),
            plugins: Some(vec![
                PluginConfig {
                    index_path: "".to_string(),
                    remote_repository: "".to_string(),
                    package_name: "".to_string(),
                },
            ]),
            git: Some(GitConfig {
                user_email: "".to_string(),
                user_name: "".to_string(),
                force_push: false,
            }),
            verbose: Some(false),
            dry_run: Some(false),
            cwd: Some("".to_string()),
        };

        testcfg
    }

    fn get_failing_test_config() -> TomlConfig {
        let testcfg = TomlConfig {
            binaries: Some(BinariesConfig {
                git: "/bin/true".to_string(),
                wpcli: "/this/should/not/work".to_string(),
            }),
            plugins: Some(vec![
                PluginConfig {
                    index_path: "".to_string(),
                    remote_repository: "".to_string(),
                    package_name: "".to_string(),
                },
            ]),
            git: Some(GitConfig {
                user_email: "".to_string(),
                user_name: "".to_string(),
                force_push: false,
            }),
            verbose: Some(false),
            dry_run: Some(false),
            cwd: Some("".to_string()),
        };

        testcfg
    }

    #[test]
    fn test_config_getters_and_setters() {
        let mut config = get_test_config();

        config.set_cwd("/foo/bar/baz".to_string());
        config.set_dry_run(true);
        config.set_verbosity(true);

        assert_eq!(config.cwd.unwrap(), "/foo/bar/baz".to_string());
        assert!(config.verbose.unwrap());
        assert!(config.dry_run.unwrap());
    }

    #[test]
    fn test_validate_config() {
        let is_valid = validate_configuration(&get_test_config()).unwrap();
        let is_not_valid = validate_configuration(&get_failing_test_config()).unwrap_or(false);

        assert!(is_valid);
        assert!(!is_not_valid);
    }

    #[test]
    fn test_validate_binary() {
        assert!(validate_binary(&String::from("/bin/true")));
        assert!(!validate_binary(&String::from("/does/not/exist")));
    }
}
