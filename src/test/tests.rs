use crate::test::files::TestFile;
use crate::test::files::TestFile::{AllTheBytes, NoPermissions, Random};
use predicates::prelude::predicate;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

fn crab_cat() -> (assert_cmd::Command, Command) {
    (
        assert_cmd::Command::cargo_bin("crab").unwrap(),
        Command::new("cat"),
    )
}

fn compare_with_cat(
    stdin: Option<TestFile>,
    flags: Vec<&str>,
    files: Vec<TestFile>,
) -> (assert_cmd::Command, Output) {
    let (mut crab, mut cat) = crab_cat();
    crab.args(&files);
    crab.args(&flags);
    cat.args(&files);
    cat.args(&flags);
    let output = if let Some(stdin) = stdin {
        crab.pipe_stdin(PathBuf::from(stdin.get()).as_path())
            .unwrap();
        cat.stdin(Stdio::from(File::open(stdin.get()).unwrap()))
    } else {
        &mut cat
    }
    .output()
    .expect("Cat doesn't exist");
    (crab, output)
}

fn std_eq(stdin: Option<TestFile>, flags: Vec<&str>, files: Vec<TestFile>) {
    let (mut crab, cat_out) = compare_with_cat(stdin, flags, files);
    crab.assert()
        .stdout(predicate::eq(cat_out.stdout.as_slice()));
}

#[test]
fn random() {
    std_eq(None, vec![], vec![Random]);
}
#[test]
fn random_numbered() {
    std_eq(None, vec!["-n"], vec![Random]);
}
#[test]
fn random_numbered_non_blank() {
    std_eq(None, vec!["-b"], vec![Random]);
}
#[test]
fn random_show_ends() {
    std_eq(None, vec!["-E"], vec![Random]);
}
#[test]
fn random_show_tabs() {
    std_eq(None, vec!["-T"], vec![Random]);
}
#[test]
fn random_squeeze_blank() {
    std_eq(None, vec!["-s"], vec![Random]);
}
#[test]
fn random_plus_random() {
    std_eq(None, vec![], vec![Random, Random]);
}
#[test]
fn random_plus_random_stdin() {
    std_eq(Some(Random), vec![], vec![Random]);
}
#[test]
fn fail_on_read() {
    let (mut crab, _) = crab_cat();
    crab.arg(NoPermissions)
        .assert()
        .failure()
        .stderr(predicate::str::contains(" Permission denied"));
}
#[test]
fn all_bytes_show_non_printing() {
    std_eq(None, vec!["-v"], vec![AllTheBytes]);
}
#[test]
fn all_bytes_show_all() {
    std_eq(None, vec!["-A"], vec![AllTheBytes]);
}
