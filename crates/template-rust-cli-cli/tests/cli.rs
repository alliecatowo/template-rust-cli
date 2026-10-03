use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn greets_by_name() {
    Command::cargo_bin("template-rust-cli")
        .unwrap()
        .arg("Allie")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Allie!"));
}

#[test]
fn rejects_blank_name() {
    Command::cargo_bin("template-rust-cli")
        .unwrap()
        .arg(" ")
        .assert()
        .failure();
}
