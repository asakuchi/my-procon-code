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
2 3
test
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "6 test\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
72
128 256
myonmyon
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "456 myonmyon\n");
    assert!(output.stderr_str().is_empty());
}
