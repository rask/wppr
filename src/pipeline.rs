//! # Pipeline
//!
//! Defines an upgrade pipeline that can be used to upgrade and gitify single
//! WordPress plugins.

use fs_extra;

use std::{
    fs::write,
    path::PathBuf
};

use config::RuntimeConfig;
use git::Git;
use wordpress::{Plugin, WpCli, get_plugin_version};

/// Data for an upgrade pipeline.
pub struct Pipeline {
    plugin: Plugin,
    has_backup: bool,
    backup_dir: PathBuf,
    git_cli: Git,
    wp_cli: WpCli,
    dry_run: bool,
    verbose: bool
}

/// Pipeline implementation.
impl Pipeline {
    /// Create a new pipeline instance.
    pub fn new(config: &RuntimeConfig, plugin: &Plugin, backup_dir: &PathBuf) -> Result<Pipeline, String> {
        if config.dry_run {
            println!("Creating dry run pipeline for plugin `{}`", plugin.get_nicename());
        }

        let nicename = plugin.get_cli_name()?;

        let mut backup_subdir: PathBuf = backup_dir.clone();
        backup_subdir.push(nicename);

        let mut plugin_dir = plugin.index_path.clone();
        plugin_dir.pop();

        let git = Git::new(config.binaries.git.clone(), config.git.clone(), plugin_dir.clone());
        let wp = WpCli::new(config.binaries.wpcli.clone(), plugin_dir.clone());

        Ok(Pipeline {
            plugin: (*plugin).clone(),
            has_backup: false,
            backup_dir: backup_subdir,
            git_cli: git,
            wp_cli: wp,
            dry_run: config.dry_run,
            verbose: config.verbose
        })
    }

    /// Output a progress log entry to stdout.
    fn progress_log(&self, msg: &str) {
        let pname = self.plugin.get_nicename();

        println!("[{}] {}", pname, msg);
    }

    /// Run the pipeline, first by maybe initing the plugin and then doing
    /// upgrades.
    pub fn run(&mut self) -> Result<bool, String> {
        self.progress_log("Starting upgrade run");

        self.maybe_initialize_plugin()?;
        self.create_backup()?;

        if self.plugin.pre_cmds.is_empty() == false {
            self.progress_log("Running plugin pre-commands before upgrade");
            self.run_pre_cmds()?;
        }

        let updated = self.update_plugin();

        match updated {
            Ok(b) => {
                match b {
                    true => (), // update was made and succeeded
                    false => {
                        self.progress_log("Plugin already up to date, proceeding");
                        return Ok(true)
                    }
                }
            },
            Err(s) => {
                self.restore_backup()?;
                self.git_cli.reset_contents()?;

                return Err(s);
            }
        }

        self.restore_backup()?; // as the upgrade removed our git and composerjson we restore them

        if self.dry_run == false && self.git_cli.has_uncommited_changes()? == false {
            // no changes done during update, we're done here
            return Ok(true);
        }

        let current_version = self.plugin.installed_version.clone().unwrap();
        let new_version = get_plugin_version(&self.plugin)?;

        if self.dry_run == false && current_version == new_version {
            self.restore_backup()?;
            self.git_cli.reset_contents()?;

            // no upgrade done, break out
            return Ok(true);
        }

        let result: Result<bool, String>;

        if self.dry_run == false {
            self.git_cli.add_and_commit_changes()?;
            self.git_cli.add_tag(new_version)?;

            result = self.git_cli.push_to_remote();
        } else {
            result = Ok(true);
        }

        match result {
            Ok(_) => {
                self.progress_log("Upgrade run finished");

                Ok(true)
            },
            Err(s) => {
                self.restore_backup()?;
                self.git_cli.reset_contents()?;

                return Err(s);
            }
        }
    }

    /// Run plugin-defined pre-commands. They are just shell commands defined in
    /// the WPPR config.
    fn run_pre_cmds(&self) -> Result<(), String> {
        unimplemented!()
    }

    fn maybe_initialize_plugin(&self) -> Result<(), String> {
        self.initialize_git_repo_for_plugin()?;
        self.create_composerjson_for_plugin()?;

        return Ok(());
    }

    fn initialize_git_repo_for_plugin(&self) -> Result<(), String> {
        self.progress_log("Initializing git repo if one does not exist");

        if self.dry_run {
            return Ok(());
        }

        let git_inited = match self.git_cli.repository_is_initialized()? {
            true => true,
            false => self.git_cli.initialize_repository()? && self.git_cli.set_repo_git_config()?
        };

        if git_inited == false {
            return Err(format!(
                "Failed to initialize git repository for plugin `{}`",
                self.plugin.get_nicename()
            ));
        }

        self.git_cli.add_remote(self.plugin.remote_repository.clone())?;
        self.git_cli.add_and_commit_changes()?; // add the initial contents

        return Ok(());
    }

    fn create_composerjson_for_plugin(&self) -> Result<(), String> {
        self.progress_log("Creating composer.json if it does not exist");

        if self.dry_run {
            return Ok(());
        }

        let composerjson_path = self.plugin.get_composerjson_path();

        if composerjson_path.exists() {
            return Ok(());
        }

        let initial_contents = format!("{{\
    \"name\": \"{}\"\
    \"type\": \"wordpress-plugin\"\
}}", self.plugin.package_name);

        let result = write(composerjson_path, initial_contents);

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                let errstr = format!("{}", e);
                Err(errstr)
            }
        }
    }

    fn create_backup(&mut self) -> Result<(), String> {
        self.progress_log("Creating history data and config backup");

        if self.dry_run {
            return Ok(());
        }

        let git_dir = self.plugin.get_git_dir()?;
        let mut dest = self.backup_dir.clone();
        dest.push(".git");

        let mut copts = fs_extra::dir::CopyOptions::new();
        copts.copy_inside = true;
        copts.overwrite = true;

        if self.verbose {
            self.progress_log(&format!("Working with backup directory `{:?}`", self.backup_dir));
            self.progress_log(&format!("Copying files from `{:?}` to `{:?}`", git_dir, dest));
        }

        let _ = fs_extra::dir::create(&dest, true);
        let backup_result = fs_extra::dir::copy(&git_dir, &dest, &copts);

        match backup_result {
            Ok(_) => {
                let mut backedup_git = self.backup_dir.clone();
                backedup_git.push(".git");
                let gitexist = backedup_git.exists() && backedup_git.is_dir();

                if gitexist == false {
                    Err(format!(
                        "Failed to backup plugin .git directory for `{}`", self.plugin.get_nicename()
                    ))
                } else {
                    self.has_backup = true;

                    Ok(())
                }
            },
            Err(e) => Err(format!(
                "Creating backup failed for plugin `{}`: `{:?}`",
                self.plugin.get_nicename(),
                e
            ))
        }
    }

    fn restore_backup(&mut self) -> Result<(), String> {
        self.progress_log("Restoring history data and config");

        if self.dry_run {
            return Ok(());
        }

        if self.has_backup == false {
            return Err(format!(
                "Cannot restore backup for `{}`, no backup has been created yet",
                self.plugin.get_nicename()
            ));
        }

        self.create_composerjson_for_plugin()?;

        let mut backedup_gitdir = self.backup_dir.clone();
        backedup_gitdir.push(".git");

        // remove the existing git dir to make a clean copy possible
        let mut plugin_gitdir = self.plugin.get_git_dir_path();
        let _ = fs_extra::dir::create(&plugin_gitdir, true);

        let mut copts = fs_extra::dir::CopyOptions::new();
        copts.copy_inside = true;
        copts.overwrite = true;

        if self.verbose {
            self.progress_log(&format!("Working with backup directory `{:?}`", self.backup_dir));
            self.progress_log(&format!("Copying files from `{:?}` to `{:?}`", backedup_gitdir, plugin_gitdir));
        }

        plugin_gitdir.pop();

        let restore_result = fs_extra::dir::copy(&backedup_gitdir, &plugin_gitdir, &copts);

        match restore_result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Restoring backup failed for plugin `{}`: `{:?}`",
                self.plugin.get_nicename(),
                e
            ))
        }
    }

    /// Update the designated plugin via WpCli.
    fn update_plugin(&self) -> Result<bool, String> {
        self.progress_log("Running WordPress update procedure");

        if self.dry_run {
            return Ok(true);
        }

        return Ok(self.wp_cli.update_plugin(&self.plugin)?.contains("already updated") == false);
    }
}
