use assert_cmd::Command;
use indoc::indoc;

#[test]
fn present_without_index() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("name[val()]")
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            name "CI"
        "#});
}

#[test]
fn absent_identifier_present_without_index() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("does_not_exist[val()]")
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn present_with_index() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("on[val(1)]")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            on "push" "pull_request"
        "#});
}

#[test]
fn absent_identifier_present_with_index() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("does_not_exist[val(1)]")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("[val(3)]")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_idenrifier_absent() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("does_not_exist[val(3)]")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(1) = "pull_request"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            on "push" "pull_request"
        "#});
}

#[test]
fn absent_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(1) = "pull_request"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(0) != "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            name "CI"
        "#});
}

#[test]
fn absent_not_equal() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(0) != "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(0) ^= "pu"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            on "push" "pull_request"
        "#});
}

#[test]
fn absent_starts_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(0) ^= "pu"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(0) $= "sh"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            on "push" "pull_request"
        "#});
}

#[test]
fn absent_ends_with() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(0) $= "sh"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(0) *= "us"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            on "push" "pull_request"
        "#});
}

#[test]
fn absent_contains() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(0) *= "us"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(0) > "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_greater_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(0) > "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn greater_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(0) >= "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_greater_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(0) >= "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn less_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(0) < "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_less_than() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(0) < "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn less_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"[val(0) <= "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn absent_less_than_or_equal_to() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"does_not_exist[val(0) <= "push"]"#)
        .write_stdin(indoc! {r#"
            on "push" "pull_request"
            name "CI"
            jobs
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}
