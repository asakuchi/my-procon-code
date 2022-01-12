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
NYN
YNN
NNN
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n");
    assert!(output.stderr_str().is_empty());
}
#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
4
NYYY
YNNN
YNNN
YNNN
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
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
NYY
YNY
YYN
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
6
NNNNNY
NNNNYN
NNNNYN
NNNNNN
NYYNNN
YNNNNN
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "24\n");
    assert!(output.stderr_str().is_empty());
}
