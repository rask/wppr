//! testfns.rs
//!
//! Test functions, helpers, and utils.

use std::{env, fs, path::PathBuf};

/// Get the working directory where tests have been run from.
pub fn get_cwd() -> PathBuf {
    PathBuf::from(env::current_dir().unwrap().to_str().unwrap())
}

/// Get the directory that contains tests.
pub fn get_tests_dir(path: &'static str) -> PathBuf {
    let mut testspath: PathBuf = get_cwd();

    testspath.push("tests");
    testspath.push(path);

    testspath
}

pub fn setup_test_dummy_project() {
    let proj_src: PathBuf = get_tests_dir("data/testproj-src");
    let proj_dest: PathBuf = get_tests_dir("data/testproj");

    if !proj_src.is_dir() {
        panic!("Cannot setup test project, source `{:?}` missing", proj_src);
    }

    fs::remove_dir(&proj_dest);
    fs::create_dir(&proj_dest);

    fs::copy(proj_src, proj_dest);
}

pub fn update_test_dummy_project() {
    let proj_src: PathBuf = get_tests_dir("data/testproj-update");
    let proj_dest: PathBuf = get_tests_dir("data/testproj");

    if !proj_src.is_dir() {
        panic!("Cannot setup test project, source `{:?}` missing", proj_src);
    }

    fs::copy(proj_src, proj_dest);
}
