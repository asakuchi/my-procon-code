use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 3 1 2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3.000000 0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 2 1 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2.000000 1\n");
    assert!(output.stderr_str().is_empty());
}
