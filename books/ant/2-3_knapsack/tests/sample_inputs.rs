use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample0() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
3
7
3 4
4 5
2 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10\n");
    assert!(output.stderr_str().is_empty());
}
