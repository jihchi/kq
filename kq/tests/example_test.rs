// test cases are coming from https://github.com/kdl-org/kdl/blob/1.0.0/QUERY-SPEC.md#examples
use assert_cmd::Command;
use indoc::indoc;

const INPUT: &str = indoc! {r#"
    package {
        name "foo"
        version "1.0.0"
        dependencies platform="windows" {
            winapi "1.0.0" path="./crates/my-winapi-fork"
        }
        dependencies {
            miette "2.0.0" dev=true
        }
    }
"#};

#[test]
fn descendant() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("package name")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            name "foo"
        "#});
}

#[test]
fn top_child() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("top() > package name")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            name "foo"
        "#});
}

#[test]
fn descendant_by_identifier() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("dependencies")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            dependencies platform="windows" {
                winapi "1.0.0" path=".\/crates\/my-winapi-fork"
            }
            dependencies {
                miette "2.0.0" dev=true
            }
        "#});
}

#[test]
fn identifier_and_implicit_property_name() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("dependencies[platform]")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            dependencies platform="windows" {
                winapi "1.0.0" path=".\/crates\/my-winapi-fork"
            }
        "#});
}

#[test]
fn identifier_and_explicit_property_name() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("dependencies[prop(platform)]")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            dependencies platform="windows" {
                winapi "1.0.0" path=".\/crates\/my-winapi-fork"
            }
        "#});
}

#[test]
fn all_direct_children() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("dependencies > []")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            winapi "1.0.0" path=".\/crates\/my-winapi-fork"
            miette "2.0.0" dev=true
        "#});
}
