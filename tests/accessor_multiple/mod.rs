use assert_cmd::Command;
use indoc::indoc;

const INPUT: &str = include_str!("./website.kdl");

#[test]
fn top_descendant_any_element() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("top() []")
        .write_stdin(indoc! {r#"
            name "CI"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test" {
                    strategy {
                        matrix {
                            os "ubuntu-latest" "macOS-latest" "windows-latest"
                        }
                    }
                }
            }
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            name "CI"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test" {
                    strategy {
                        matrix {
                            os "ubuntu-latest" "macOS-latest" "windows-latest"
                        }
                    }
                }
            }
        "#});
}

#[test]
fn top_child_any_element() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("top() > []")
        .write_stdin(indoc! {r#"
            name "CI"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test" {
                    strategy {
                        matrix {
                            os "ubuntu-latest" "macOS-latest" "windows-latest"
                        }
                    }
                }
            }
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            name "CI"
            jobs {
                fmt_and_docs "Check fmt & build docs"
                build_and_test "Build & Test" {
                    strategy {
                        matrix {
                            os "ubuntu-latest" "macOS-latest" "windows-latest"
                        }
                    }
                }
            }
        "#});
}

#[test]
fn descendant_child() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("html > body section > h2")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            h2 "Design and Discussion"
            h2 "Design Principles"
        "#});
}

#[test]
fn general_sibling() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("html > head meta ~ title")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            title "kdl - Kat's Document Language"
        "#});
}

#[test]
fn adjacent_sibling() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("html body h2 + ol")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            ol {
                li "Maintainability"
                li "Flexibility"
                li "Cognitive simplicity and Learnability"
                li "Ease of de\/serialization"
                li "Ease of implementation"
            }
        "#});
}

#[test]
fn general_adjacent_siblings() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("html > head meta ~ title + link")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(predicates::str::starts_with("link"))
        .stdout(predicates::str::contains(r#"href="\/styles\/global.css""#))
        .stdout(predicates::str::contains(r#"rel="stylesheet""#));
}

#[test]
fn adjacent_general_siblings() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg("html > head meta + meta ~ link")
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(predicates::str::starts_with("link"))
        .stdout(predicates::str::contains(r#"href="\/styles\/global.css""#))
        .stdout(predicates::str::contains(r#"rel="stylesheet""#));
}

#[test]
fn complex_single_level() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"li[val() = "Flexibility"] + [val() = "Cognitive simplicity and Learnability"] ~ [val() = "Ease of implementation"]"#)
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            li "Ease of implementation"
        "#});
}

#[test]
fn complex_nested() {
    Command::cargo_bin("kq")
        .unwrap()
        .arg(r#"header + section[prop(id) = "description"] ~ section[class = "kdl-section"] ol > li"#)
        .write_stdin(INPUT)
        .assert()
        .success()
        .stdout(indoc! {r#"
            li "Maintainability"
            li "Flexibility"
            li "Cognitive simplicity and Learnability"
            li "Ease of de\/serialization"
            li "Ease of implementation"
        "#});
}
