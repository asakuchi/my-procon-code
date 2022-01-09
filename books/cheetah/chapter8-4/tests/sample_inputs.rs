use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
6800
100
68
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1.3322616182218813E-13\n");
    assert!(output.stderr_str().is_empty());
}
#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
2000
510
4
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9.56205462458368\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
15000
364
48
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "7.687856394581649\n");
    assert!(output.stderr_str().is_empty());
}
