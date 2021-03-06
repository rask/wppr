//! testfns.rs
//!
//! Test functions, helpers, and utils.

extern crate fs_extra;

use std::{process, env, path::PathBuf};

use self::fs_extra::dir;

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

    if !proj_src.exists() {
        panic!("Cannot setup test project, source `{:?}` missing", proj_src);
    }

    let mut copts = dir::CopyOptions::new();
    copts.overwrite = true;
    copts.copy_inside = true;

    match dir::copy(proj_src, &proj_dest, &copts) {
        Err(e) => {
            panic!("Could not setup test dummy project: `{:?}`", e);
        },
        _ => ()
    };

    if !proj_dest.exists() {
        panic!("Could not setup test dummy project: directory `{}` was not created", proj_dest.display());
    }
}

pub fn update_test_dummy_project() {
    let proj_src: PathBuf = get_tests_dir("data/testproj-update");
    let proj_dest: PathBuf = get_tests_dir("data/testproj");

    if !proj_src.exists() {
        panic!("Cannot update test project, source `{:?}` missing", proj_src);
    }

    if !proj_dest.exists() {
        panic!("Cannot update test project, destination `{:?}` missing", proj_dest);
    }

    let mut copts = dir::CopyOptions::new();
    copts.overwrite = true;
    copts.copy_inside = true;
    copts.skip_exist = false;

    let copyfrom = proj_src.to_str().unwrap();
    let copyto = proj_dest.to_str().unwrap();

    let result = dir::copy(&copyfrom, &copyto, &copts);

    match result {
        Ok(_) => (),
        Err(e) => panic!("Could not update test project: `{}`", e)
    };

    let proj_dest_testfile: PathBuf = get_tests_dir("data/testproj/plugins/testone/additional.php");

    if proj_dest_testfile.exists() == false {
        panic!("Could not update test project!");
    }
}

pub fn discard_test_dummy_project() {
    let proj_dest: PathBuf = get_tests_dir("data/testproj");

    process::Command::new("rm")
        .current_dir(get_cwd())
        .arg("-rf")
        .arg(&proj_dest)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if proj_dest.exists() {
        panic!("Could not remove test project!");
    }
}
