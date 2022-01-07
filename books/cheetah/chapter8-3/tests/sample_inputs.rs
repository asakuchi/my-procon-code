use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
4
400 100 100 100
Danny_Messer Stella_Bonasera Stella_Bonasera Mac_Taylor
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[3, 1, 2, 0]\n");
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
200 200 200
Gil Sarah Warrick
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[0, 1, 2]\n");
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
100 200 50
Horatio_Caine horatio_caine YEAAAAH
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "[2, 0, 1]\n");
    assert!(output.stderr_str().is_empty());
}
