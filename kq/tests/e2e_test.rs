use assert_cmd::Command;
use predicates::prelude::*;

mod accessor_multiple;
mod accessor_single;

#[test]
fn sanity() {
    Command::cargo_bin("kq")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::contains("print this help menu"))
        .stdout(predicate::str::contains("print the version"));
}
