use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
2
3
100
400
200
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1400\n");
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
3
100
400
300
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1600\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
5
5
464
464
464
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4640\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
1
4
20
-30
-10
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-100\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
9
1
-1
-10
4
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}
