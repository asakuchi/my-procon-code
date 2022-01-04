use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
3
...
...
...
0
1
4
1 0 -1 0
0 1 0 -1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n");
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
...
...
...
0
1
8
1 0 -1 0 1 1 -1 -1
0 1 0 -1 1 -1 1 -1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            4
            X.X
            ...
            XXX
            X.X
            0
            1
            4
            1 0 -1 0
            0 1 0 -1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            6
            .......
            X.X.X..
            XXX...X
            ....X..
            X....X.
            .......
            5
            0
            6
            1 0 -1 0 -2 1
            0 -1 0 1 3 0
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            1
            .......
            0
            0
            6
            1 0 1 0 1 0
            0 1 0 1 0 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "6\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample6() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            1
            ..X.X.X.X.X.X.
            0
            0
            4
            2 0 -2 0
            0 2 0 -2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1\n");
    assert!(output.stderr_str().is_empty());
}
