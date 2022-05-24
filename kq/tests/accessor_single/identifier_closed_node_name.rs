use assert_cmd::Command;
use indoc::indoc;

#[test]
fn parentheses() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("step[name()]")
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
        .arg(r#"profile[name() = "profile"]"#)
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
        .stdout(indoc! {r#"
            profile "minimal"
        "#});
}

#[test]
fn absent_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[name() = "profile"]"#)
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
fn not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"profile[name() != "step"]"#)
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
        .stdout(indoc! {r#"
            profile "minimal"
        "#});
}

#[test]
fn absent_not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"nonsense[name() != "step"]"#)
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
fn starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"profile[name() ^= "pro"]"#)
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
        .stdout(indoc! {r#"
            profile "minimal"
        "#});
}

#[test]
fn absent_starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[name() ^= "pro"]"#)
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
fn ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"profile[name() $= "file"]"#)
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
        .stdout(indoc! {r#"
            profile "minimal"
        "#});
}

#[test]
fn absent_ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[name() $= "file"]"#)
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
fn contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"profile[name() *= "rofil"]"#)
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
        .stdout(indoc! {r#"
            profile "minimal"
        "#});
}

#[test]
fn absent_contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"step[name() *= "rofil"]"#)
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
fn greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"profile[name() > "profile"]"#)
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
fn absent_greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_matter[name() > "profile"]"#)
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
fn greater_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"profile[name() >= "profile"]"#)
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
fn absent_greater_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_matter[name() >= "profile"]"#)
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
fn less_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"profile[name() < "profile"]"#)
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
fn absent_less_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_matter[name() < "profile"]"#)
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
fn less_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"profile[name() <= "profile"]"#)
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
fn absent_less_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_matter[name() <= "profile"]"#)
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
