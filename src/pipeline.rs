//! # Pipeline
//!
//! Defines an upgrade pipeline that can be used to upgrade and gitify single
//! WordPress plugins.

use fs_extra;

use std::{
    fs::{write, remove_dir_all},
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
}

/// Pipeline implementation.
impl Pipeline {
    /// Create a new pipeline instance.
    pub fn new(config: &RuntimeConfig, plugin: &Plugin, backup_dir: &PathBuf) -> Pipeline {
        let nicename = plugin.nicename.clone().unwrap();

        let mut backup_subdir: PathBuf = backup_dir.clone();
        backup_subdir.push(nicename);

        let mut plugin_dir = plugin.index_path.clone();
        plugin_dir.pop();

        let git = Git::new(config.binaries.git.clone(), config.git.clone(), plugin_dir.clone());
        let wp = WpCli::new(config.binaries.wpcli.clone(), plugin_dir.clone());

        Pipeline {
            plugin: (*plugin).clone(),
            has_backup: false,
            backup_dir: backup_subdir,
            git_cli: git,
            wp_cli: wp
        }
    }

    /// Run the pipeline, first by maybe initing the plugin and then doing
    /// upgrades.
    pub fn run(&mut self) -> Result<bool, String> {
        self.maybe_initialize_plugin()?;
        self.create_backup()?;

        let updated = self.update_plugin();

        match updated {
            Ok(_) => (),
            Err(s) => {
                self.restore_backup()?;
                self.git_cli.reset_contents()?;

                return Err(s);
            }
        }

        self.restore_backup()?; // as the upgrade removed our git and composerjson we restore them

        if self.git_cli.has_uncommited_changes()? == false {
            // no changes done during update, we're done here
            return Ok(true);
        }

        let current_version = self.plugin.installed_version.clone().unwrap();
        let new_version = get_plugin_version(&self.plugin);

        if current_version == new_version {
            // OK what just happened
            self.restore_backup()?;
            self.git_cli.reset_contents()?;

            return Err(format!(
                "Unknown error, upgrade resulted in same installed version in plugin `{}`, aborting",
                self.plugin.get_nicename()
            ));
        }

        self.git_cli.add_and_commit_changes()?;
        self.git_cli.add_tag(new_version)?;

        let result = self.git_cli.push_to_remote();

        match result {
            Ok(_) => Ok(true),
            Err(s) => {
                self.restore_backup()?;
                self.git_cli.reset_contents()?;

                return Err(s);
            }
        }
    }

    fn maybe_initialize_plugin(&self) -> Result<(), String> {
        self.initialize_git_repo_for_plugin()?;
        self.create_composerjson_for_plugin()?;

        return Ok(());
    }

    fn initialize_git_repo_for_plugin(&self) -> Result<(), String> {
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
        let composerjson_path = self.plugin.get_composerjson_file()?;

        if composerjson_path.exists() {
            return Ok(()); // assume a proper composer.json already exists and continue
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
        let git_dir = self.plugin.get_git_dir()?;
        let composerjson_file = self.plugin.get_composerjson_file()?;

        let copy_from = vec![git_dir, composerjson_file];

        let mut copts = fs_extra::dir::CopyOptions::new();
        copts.copy_inside = true;
        copts.overwrite = true;

        let backup_result = fs_extra::copy_items(&copy_from, &self.backup_dir, &copts);

        match backup_result {
            Ok(_) => {
                self.has_backup = true;

                Ok(())
            },
            Err(_) => Err(format!(
                "Creating backup failed for plugin `{}`, please check permissions and try again",
                self.plugin.get_nicename()
            ))
        }
    }

    fn restore_backup(&mut self) -> Result<(), String> {
        if self.has_backup == false {
            return Err(format!(
                "Cannot restore backup for `{}`, no backup has been created yet",
                self.plugin.get_nicename()
            ));
        }

        let mut backedup_gitdir = self.backup_dir.clone();
        let mut backedup_composerjsonfile = self.backup_dir.clone();

        let mut dest = self.plugin.index_path.clone();
        dest.pop();

        backedup_gitdir.push(".git");
        backedup_composerjsonfile.push("composer.json");

        let restore = vec![backedup_gitdir, backedup_composerjsonfile];

        let mut copts = fs_extra::dir::CopyOptions::new();
        copts.copy_inside = true;
        copts.overwrite = true;

        // remove the existing git dir to make a clean copy possible
        let plugin_gitdir = self.plugin.get_git_dir();

        match plugin_gitdir {
            Ok(d) => {
                remove_dir_all(&d).expect(
                    &format!(
                        "Could not remove existing .git directory for plugin `{}` while restoring from backup",
                        self.plugin.get_nicename()
                    )
                );
            },
            Err(_) => {}
        };

        let restore_result = fs_extra::copy_items(&restore, &dest, &copts);

        match restore_result {
            Ok(_) => Ok(()),
            Err(_) => Err(format!(
                "Restoring backup failed for plugin `{}`, please check permissions and try again",
                self.plugin.get_nicename()
            ))
        }
    }

    /// Update the designated plugin via WpCli.
    fn update_plugin(&self) -> Result<bool, String> {
        return self.wp_cli.update_plugin(&self.plugin);
    }
}
