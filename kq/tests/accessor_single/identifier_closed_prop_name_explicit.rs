use assert_cmd::Command;
use indoc::indoc;

#[test]
fn present_prop() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("step[prop(uses)]")
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            step uses="actions\/checkout@v1"
            step uses="actions-rs\/toolchain@v1"
        "#});
}

#[test]
fn absent_prop() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("step[prop(does_not_exist)]")
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_identifier() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("does_not_exist[prop(uses)]")
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) = "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            step uses="actions\/checkout@v1"
        "#});
}

#[test]
fn absent_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) = "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) != "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            step uses="actions-rs\/toolchain@v1"
        "#});
}

#[test]
fn absent_not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) != "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) ^= "actions"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            step uses="actions\/checkout@v1"
            step uses="actions-rs\/toolchain@v1"
        "#});
}

#[test]
fn absent_starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) ^= "actions"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) $= "@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            step uses="actions\/checkout@v1"
            step uses="actions-rs\/toolchain@v1"
        "#});
}

#[test]
fn absent_ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) $= "@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) *= "toolchain"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            step uses="actions-rs\/toolchain@v1"
        "#});
}

#[test]
fn absent_contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) *= "toolchain"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) > "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) > "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn greater_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) >= "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_greater_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) >= "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn less_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) < "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_less_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) < "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn less_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[prop(uses) <= "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_less_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[prop(uses) <= "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}
