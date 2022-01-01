use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
10
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[3, 9]\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[2]\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
9
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[2, 4, 8]\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
26
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[5, 25]\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
30
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[29]\n");
    assert!(output.stderr_str().is_empty());
}
