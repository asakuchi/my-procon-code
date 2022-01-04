// use cli_test_dir::*;

// const BIN: &'static str = "./main";

// #[test]
// fn sample1() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(
//             r#"
// YNYY
// "#,
//         )
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "5\n");
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn sample2() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(
//             r#"
// YNNN
// "#,
//         )
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "8\n");
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn sample3() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(
//             r#"
// NNNN
// "#,
//         )
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "16\n");
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn sample4() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(
//             r#"
// YYYY
// "#,
//         )
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "1\n");
//     assert!(output.stderr_str().is_empty());
// }

// #[test]
// fn sample5() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(
//             r#"
// NYNY
// "#,
//         )
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "11\n");
//     assert!(output.stderr_str().is_empty());
// }
