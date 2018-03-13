extern crate tempdir;

use self::tempdir::TempDir;

use std::process::Command;

#[test]
fn ffi_c() {
    let tmpdir = TempDir::new("zbox_test_ffi").expect("Create temp dir failed");
    let output_dir = tmpdir.path();
    let exe = output_dir.join("ffi");

    let mut cmd = Command::new("gcc");
    cmd.arg("-o")
        .arg(&exe)
        .arg("tests/ffi.c")
        .arg("-lzbox")
        .arg("-Isrc/ffi/include");

    if cfg!(debug_assertions) {
        cmd.arg("-Ltarget/debug");
    } else {
        cmd.arg("-Ltarget/release");
    }

    // compile
    let output = cmd.output().expect("Failed to run gcc");
    if !output.status.success() {
        println!("{:#?}", output);
    }
    assert!(output.status.success());

    // execute
    let output = Command::new(&exe).output().expect("Failed to run ffi");
    if !output.status.success() {
        println!("{:#?}", output);
    }
    assert!(output.status.success());
}