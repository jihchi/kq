use assert_cmd::Command;
use indoc::indoc;

mod closed_node_name;
mod closed_prop_name_explicit;
mod closed_prop_name_implicit;
mod closed_props;
mod closed_type_tag;
mod closed_val;
mod closed_values;
mod identifier_closed_node_name;
mod identifier_closed_prop_name_explicit;
mod identifier_closed_prop_name_implicit;
mod identifier_closed_props;
mod identifier_closed_type_tag;
mod identifier_closed_val;
mod identifier_closed_values;

#[test]
fn top() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("top()")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test"
            }
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test"
            }
        "#});
}

#[test]
fn type_tag() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("()")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test"
            }
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn type_type_with_identifier() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("(jobs)")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test"
            }
        "#})
        .assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn sole() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("jobs")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test"
            }
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test"
            }
        "#});
}

#[test]
fn any_element() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("[]")
        .write_stdin(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test"
            }
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            name "CI"
            on "push" "pull_request"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test"
            }
        "#});
}
