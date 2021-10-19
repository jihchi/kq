use assert_cmd::Command;
use indoc::indoc;

#[test]
fn present() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("[uses]")
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            step uses="actions\/checkout@v1"
        "#});
}

#[test]
fn absent() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("[does_not_exist]")
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[uses = "actions/checkout@v1"]"#)
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
fn not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[uses != "actions/checkout@v1"]"#)
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
fn starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[uses ^= "actions"]"#)
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
fn ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[uses $= "@v1"]"#)
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
fn contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[uses *= "s/"]"#)
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
fn greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[uses > "actions/checkout@v1"]"#)
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
        .arg(r#"[uses >= "actions/checkout@v1"]"#)
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
        .arg(r#"[uses < "actions/checkout@v1"]"#)
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
        .arg(r#"[uses <= "actions/checkout@v1"]"#)
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step run="cargo test --all --verbose"
            step uses="actions-rs/toolchain@v1"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}
