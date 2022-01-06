use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
1000
0
2
3
10 20 30
15 24 32
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "500\n");
    assert!(output.stderr_str().is_empty());
}
#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
1000
0
2
1
10
9
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
100
20
8
3
40 50 60
37 48 55
100 48 50
105 48 47
110 50 52
110 50 52
110 51 54
109 49 53
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "239\n");
    assert!(output.stderr_str().is_empty());
}
