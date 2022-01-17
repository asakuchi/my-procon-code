use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample0() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
5
-1 3 4 4 7
3 -1 5 -1 6
4 5 -1 5 -1
4 -1 5 -1 3
7 6 -1 3 -1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}
