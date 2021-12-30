use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
2
1
20 20
5 8
0
1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[0, 13]\n");
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
1
10 10
5 8
0
1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[3, 10]\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
3
3
30 20 10
10 5 5
0 1 2
1 2 0
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[10, 10, 0]\n");
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
8
14 35 86 58 25 62
6 34 27 38 9 60
1 2 4 5 3 3 1 0
0 1 2 4 2 5 3 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[0, 14, 65, 35, 25, 35]\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
4
5
700000 800000 900000 1000000
478478 478478 478478 478478
2 3 2 0 1
0 1 1 3 2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[0, 156956, 900000, 856956]\n");
    assert!(output.stderr_str().is_empty());
}
