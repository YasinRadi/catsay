use std::{
    error::Error,
    process::Command
};
use assert_cmd::prelude::*;
use predicates::prelude::*;

const CRATE_NAME: &str = "catsay";

#[test]
fn run_with_defaults() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin(CRATE_NAME)
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin(CRATE_NAME)
        .expect("binary exists")
        .args(&["-f", "dogfile.txt"])
        .assert()
        .failure();
    Ok(())
}