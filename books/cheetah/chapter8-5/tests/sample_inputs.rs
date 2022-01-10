use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample0() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
1
0
0
2
-5 1
5 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}
#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
3
0 -6 6
0 1 2
2 2 2
-5 1
5 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
7
1 -3 2 5 -4 12 12
1 -1 2 5 5 1 1
8 1 2 1 1 1 2
-5 1
12 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
8
-3 2 2 0 -4 12 12 12
-1 2 3 1 5 1 1 1
1 3 1 7 1 1 2 3
2 3
13 2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
12
-107 -38 140 148 -198 172 -179 148 176 153 -56 -187
175 -115 23 -2 -49 -151 -52 42 0 68 109 -174
135 42 70 39 89 39 43 150 10 120 16 8
102 16
19 -108
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n");
    assert!(output.stderr_str().is_empty());
}
