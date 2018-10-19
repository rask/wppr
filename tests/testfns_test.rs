#[path = "./testfns.rs"]
mod testfns;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use testfns::*;

#[test]
fn test_dummy_project_can_be_setup() {
    let actualdest: PathBuf = PathBuf::from(
        env::current_dir().unwrap().to_str().unwrap().to_string() + "/tests/data/testproj"
    );

    let proj_dest: PathBuf = get_tests_dir("data/testproj");

    assert_eq!(actualdest, proj_dest);

    setup_test_dummy_project();

    assert!(proj_dest.exists());
    assert!(proj_dest.join("plugins/testone/testone.php").exists());

    let mut contents: String = String::new();

    File::open(proj_dest.join("plugins/testone/testone.php"))
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    assert!(contents.contains("* Version: 1.0.0"));

    discard_test_dummy_project();

    assert_eq!(false, proj_dest.exists());
}

#[test]
fn test_dummy_project_can_be_updated() {
    let actualdest: PathBuf = PathBuf::from(
        env::current_dir().unwrap().to_str().unwrap().to_string() + "/tests/data/testproj"
    );

    let proj_dest: PathBuf = get_tests_dir("data/testproj");
    let mut proj_dest_index = proj_dest.clone();
    proj_dest_index.push("plugins");
    proj_dest_index.push("testone");
    proj_dest_index.push("testone.php");

    assert_eq!(actualdest, proj_dest);

    setup_test_dummy_project();

    assert!(proj_dest_index.exists());
    assert_eq!(false, proj_dest_index.is_dir(), "{:?} is actually a directory", proj_dest_index);

    let mut contents: String = String::new();

    File::open(&proj_dest_index)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    assert!(contents.contains("* Version: 1.0.0"));

    update_test_dummy_project();

    File::open(&proj_dest_index)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    assert!(contents.contains("* Version: 1.2.3"));

    discard_test_dummy_project();

    assert_eq!(false, proj_dest.exists());
}

#[test]
fn test_dummy_project_can_be_discarded() {
    let actualdest: PathBuf = PathBuf::from(
        env::current_dir().unwrap().to_str().unwrap().to_string() + "/tests/data/testproj"
    );

    let proj_dest: PathBuf = get_tests_dir("data/testproj");

    assert_eq!(actualdest, proj_dest);

    setup_test_dummy_project();

    assert!(proj_dest.exists());

    discard_test_dummy_project();

    assert_eq!(false, proj_dest.exists());
}
