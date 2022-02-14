use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
4 3 1
1 2 2
2 3 9
2 4 5
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "16\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
5 6 2
1 2 5
1 3 3
2 3 4
2 5 7
3 4 6
4 5 5
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "12\n");
    assert!(output.stderr_str().is_empty());
}
