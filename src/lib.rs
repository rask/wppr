//! WordPress Plugin Repofier
//!
//! This tool helps WordPress developers by allowing them to install paid or
//! private WordPress plugins using normal Composer workflows.

// Copyright 2018 Otto Rask
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy
// of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

extern crate clap;
#[macro_use]
extern crate prettytable;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod wordpress;
mod git;
pub mod config;
mod commands;

use config::{validate_configuration, Config};

/// Get the application clap config.
fn get_app_init_config() -> ArgMatches<'static> {
    App::new("wppr")
        .version("0.1.0-alpha")
        .author("Otto Rask")
        .about(
            "\n[WordPress Plugin Repofier]\n\nTakes Composer unfriendly WordPress plugins and \
             generates tagged releases into a Git repository from them.",
        )
        .arg(
            Arg::with_name("config")
                .long("configuration")
                .help("Absolute path to a TOML configuration file to use")
                .takes_value(true)
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .help("Make output more verbose, useful for debugging and so on"),
        )
        .arg(
            Arg::with_name("dryrun")
                .long("dry-run")
                .takes_value(false)
                .help("Run operations without actually making changes"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List plugins being managed by chosen configuration"),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run the tool: updates, tags, and pushes changes for managed plugins"),
        )
        .get_matches()
}

/// Get application runtime configuration which has been read from a provided
/// TOML configuration file.
fn get_app_run_config(init_config: &ArgMatches) -> Result<Config, String> {
    let verbose: bool = init_config.is_present("verbosity");
    let dry_run: bool = init_config.is_present("dryrun");
    let config_file: &str = init_config.value_of("config").unwrap();

    let cfg_path = Path::new(config_file);

    if !cfg_path.is_absolute() {
        return Err("Configuration file must be given as an absolute path".to_string());
    }

    if !cfg_path.exists() {
        return Err(
            "Invalid configuration, please validate the configuration file exists".to_string(),
        );
    }

    let config_cwd: &str = match cfg_path.parent().unwrap().to_str() {
        Some(s) => s,
        None => {
            return Err(
                "Invalid configuration location given, please validate your config is in an \
                 accessable location"
                    .to_string(),
            );
        }
    };

    let mut cfg_data: String = String::new();

    File::open(config_file)
        .unwrap()
        .read_to_string(&mut cfg_data)
        .unwrap();

    let mut configuration: Config = toml::from_str(&cfg_data).unwrap();

    configuration.set_cwd(config_cwd.to_string());

    if verbose == true {
        configuration.set_verbosity(true);
    }

    if dry_run == true {
        configuration.set_dry_run(true);
    }

    let result: Result<Config, String> = match validate_configuration(&configuration) {
        Ok(_) => Ok(configuration),
        Err(estring) => Err(format!("Invalid configuration: {}", estring)),
    };

    result
}

/// Run the `list` command of this tool.
fn run_list_command(config: Config) -> Result<bool, &'static str> {
    commands::list(config)
}

/// Run the `run` command of this tool.
fn run_run_command(config: Config) -> Result<bool, &'static str> {
    commands::run(config)
}

/// Run the application. Returns an interger for exit coding.
pub fn run() -> i32 {
    let app_config: ArgMatches = get_app_init_config();
    let runtime_configuration = get_app_run_config(&app_config);

    let configuration: Config = match runtime_configuration {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    if configuration.verbose.unwrap_or(false) == true {
        println!("Configuration: {:?}", configuration);
    };

    let command_result = match app_config.subcommand_name() {
        Some("list") => run_list_command(configuration),
        Some("run") => run_run_command(configuration),
        _ => {
            eprintln!("Invalid command given");
            return 1;
        }
    };

    match command_result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    0
}
