use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
1
25
25
25
25
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1.0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
2
25
25
25
25
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.75\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
7
50
0
0
50
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1.0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
14
50
50
0
0
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1.220703125E-4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
14
25
25
25
25
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.008845493197441101\n");
    assert!(output.stderr_str().is_empty());
}
