use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
6
10 3 2 5 7 8
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "19\n");
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
11 15
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "15\n");
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
7 7 7 7 7 7 7
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "21\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
10
1 2 3 4 5 1 2 3 4 5
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "16\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
40
94 40 49 65 21 21 106 80 92 81 679 4 61
6 237 12 72 74 29 95 265 35 47 1 61 397
52 72 37 51 1 81 45 435 7 36 57 86 81 72
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2926\n");
    assert!(output.stderr_str().is_empty());
}
