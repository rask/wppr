//! # config
//!
//! Configuration related functionalities.

use std::process::{Command, Stdio};

//LCOV_EXCL_START
/// Struct to hold deserialized TOML configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub binaries: BinariesConfig,
    pub git: GitConfig,
    pub plugins: Option<Vec<PluginConfig>>,
    pub verbose: Option<bool>,
    pub dry_run: Option<bool>,
    pub cwd: Option<String>,
}

/// Configuration of binaries used when running the tool.
#[derive(Debug, Deserialize)]
pub struct BinariesConfig {
    pub git: String,
    pub wpcli: String,
}

/// Configuration for Git when running the tool.
#[derive(Debug, Deserialize)]
pub struct GitConfig {
    pub user_name: String,
    pub user_email: String,
    pub force_push: Option<bool>,
}

/// Plugins configuration when running the tool.
#[derive(Debug, Deserialize)]
pub struct PluginConfig {
    pub index_path: String,
    pub package_name: String,
    pub remote_repository: String,
}
//LCOV_EXCL_STOP

impl Config {
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
pub fn validate_configuration(config: &Config) -> Result<bool, &'static str> {
    let git_is_valid = validate_binary(&config.binaries.git);
    let wp_cli_is_valid = validate_binary(&config.binaries.wpcli);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> Config {
        let testcfg = Config {
            binaries: BinariesConfig {
                git: "/bin/true".to_string(),
                wpcli: "/bin/true".to_string(),
            },
            plugins: Some(vec![
                PluginConfig {
                    index_path: "".to_string(),
                    remote_repository: "".to_string(),
                    package_name: "".to_string(),
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

    fn get_failing_test_config() -> Config {
        let testcfg = Config {
            binaries: BinariesConfig {
                git: "/bin/true".to_string(),
                wpcli: "/this/should/not/work".to_string(),
            },
            plugins: Some(vec![
                PluginConfig {
                    index_path: "".to_string(),
                    remote_repository: "".to_string(),
                    package_name: "".to_string(),
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
