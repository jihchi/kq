use assert_cmd::Command;
use indoc::indoc;

#[test]
fn parentheses() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("step[values()]")
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step "Install Rust" uses="actions-rs/toolchain@v1" {
                profile "minimal"
            }
            step "Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_identifier() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("does_not_exist[values()]")
        .write_stdin(indoc! {r#"
            step uses="actions/checkout@v1"
            step "Install Rust" uses="actions-rs/toolchain@v1" {
                profile "minimal"
            }
            step "Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() = "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() = "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() != "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() != "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() ^= "asc"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() ^= "asc"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() $= "cii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() $= "cii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() *= "sci"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() *= "sci"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() > "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() > "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn greater_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() >= "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_greater_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() >= "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn less_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() < "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_less_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() < "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn less_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[values() <= "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_less_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[values() <= "ascii"]"#)
        .write_stdin(indoc! {r#"
            step (ascii)"Clippy" run="cargo clippy --all -- -D warnings"
            step "Run tests" run="cargo test --all --verbose"
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}
