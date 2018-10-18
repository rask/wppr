//! # git
//!
//! This module contains Git specific functionalities.

use std::{
    path::PathBuf,
    process::Command
};

use config::GitConfig;

/// Wrapper for Git.
pub struct Git {
    bin: String,
    config: GitConfig,
    working_directory: PathBuf
}

pub type GitResult = Result<bool, String>;

impl Git {
    /// Get a new git wrapper instance.
    pub fn new(bin: String, cfg: GitConfig, cwd: PathBuf) -> Self {
        Git {
            bin: bin,
            config: cfg,
            working_directory: cwd
        }
    }

    /// Get a base command for all other commands to expand upon.
    fn get_base_cmd(&self) -> Command {
        let bin = self.bin.clone();
        let cwd = self.working_directory.clone();

        let mut cmd = Command::new(bin);

        cmd.current_dir(cwd);

        return cmd;
    }

    /// Set the name and email config for a git repo.
    pub fn set_repo_git_config(&self) -> GitResult {
        let mut name_cmd = self.get_base_cmd();

        name_cmd.args(&[
            "config",
            "user.name",
            &format!("\"{}\"", self.config.user_name)
        ]);

        let mut email_cmd = self.get_base_cmd();

        email_cmd.args(&[
            "config",
            "user.name",
            &format!("\"{}\"", self.config.user_email)
        ]);

        let _ = name_cmd.output().expect("Error when trying to run git command: `git config user.name ...`");
        let _ = email_cmd.output().expect("Error when trying to run git command: `git config user.email ...`");

        return Ok(true);
    }

    /// See if a git repository is initialized.
    pub fn repository_is_initialized(&self) -> GitResult {
        let mut cmd = self.get_base_cmd();

        cmd.arg("status");

        let output = cmd.output().expect("Error when trying to run git command: `git status`");

        // exit code 0 usually means we are in fact inside a repo directory
        return Ok(output.status.success());
    }

    /// Initialize a new git repository.
    pub fn initialize_repository(&self) -> GitResult {
        let mut cmd = self.get_base_cmd();

        cmd.args(&["init", "."]);

        let output = cmd.output().expect("Error when trying to run git command: `git init .`");

        match output.status.success() {
            true => {
                self.set_repo_git_config()
            },
            false => {
                Err(format!("Could not initialize new git repository: `{}`", String::from_utf8_lossy(&output.stderr)))
            }
        }
    }

    /// Does a repo have any uncommitted changes?
    pub fn has_uncommited_changes(&self) -> GitResult {
        let mut cmd = self.get_base_cmd();

        cmd.arg("status");

        let output = cmd.output().expect("Error when trying to run git command: `git status`");

        match output.status.success() {
            false => Err(format!("Could not check git repo status: `{}`", String::from_utf8_lossy(&output.stderr))),
            true => {
                let stdout = String::from_utf8_lossy(&output.stdout);

                if stdout.contains("nothing to commit, working tree clean") {
                    // no changes
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
        }
    }

    /// If there are uncommited changes in a repo, add and commit them using a
    /// generic commit message.
    pub fn add_and_commit_changes(&self) -> GitResult {
        if self.has_uncommited_changes()? == false {
            return Ok(false);
        }

        let mut cmd = self.get_base_cmd();

        cmd.args(&["add", "."]);

        let output = cmd.output().expect("Error when trying to run git command: `git add .`");

        match output.status.success() {
            true => (),
            false => {
                return Err(format!("Could not read repository status: `{}`", String::from_utf8_lossy(&output.stderr)));
            }
        };

        let mut commit_cmd = self.get_base_cmd();

        commit_cmd.args(&[
            "commit",
            "-m",
            &format!("\"{}\"", "Automated commit by wppr")
        ]);

        let commit_output = commit_cmd.output().expect("Error when trying to run git command: `git commit -m ...`");

        match commit_output.status.success() {
            true => Ok(true),
            false => {
                Err(format!("Could not commit changes: `{}`", String::from_utf8_lossy(&output.stderr)))
            }
        }
    }

    /// Add a remote repository.
    pub fn add_remote(&self, remote_uri: String) -> GitResult {
        let mut cmd = self.get_base_cmd();

        cmd.args(&[
            "remote",
            "add",
            "wppr",
            &remote_uri,
        ]);

        let output = cmd.output().expect("Error when trying to run git command: `git remote add wppr ...`");

        match output.status.success() {
            true => Ok(true),
            false => {
                Err(format!("Could not add remote repository: `{}`", String::from_utf8_lossy(&output.stderr)))
            }
        }
    }

    /// Push changes to a remote repository.
    pub fn push_to_remote(&self) -> GitResult {
        let mut cmd = self.get_base_cmd();

        cmd.args(&["push", "wppr", "master", "--follow-tags"]);

        if self.config.force_push {
            cmd.arg("--force");
        }

        let output = cmd.output()
            .expect("Error when trying to run git command: `git push wppr master`");

        match output.status.success() {
            true => Ok(true),
            false => {
                Err(format!(
                    "Could not push to remote repository: `{}`",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
    }

    /// Add a new git tag to the repository
    pub fn add_tag(&self, tag: String) -> GitResult {
        let mut cmd = self.get_base_cmd();

        cmd.args(&["tag", &tag]);

        let output = cmd.output().expect("Error when trying to run git command: `git tag ...`");

        match output.status.success() {
            true => Ok(true),
            false => {
                Err(format!("Could not add git tag: `{}`", String::from_utf8_lossy(&output.stderr)))
            }
        }
    }

    pub fn reset_contents(&self) -> GitResult {
        let mut cmd = self.get_base_cmd();

        cmd.args(&["reset", "--hard"]);

        let output = cmd.output()
            .expect("Error when trying to run git command: `git reset --hard`");

        match output.status.success() {
            true => Ok(true),
            false => {
                Err(format!(
                    "Could not reset plugin contents to previous state: `{}`",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
    }
}