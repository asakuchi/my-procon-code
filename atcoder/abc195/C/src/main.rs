use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    if n >= 1_000 {
        result += n - 999;
    }

    if n >= 1_000_000 {
        result += n - 999_999;
    }

    if n >= 1_000_000_000 {
        result += n - 999_999_999;
    }

    if n >= 1_000_000_000_000 {
        result += n - 999_999_999_999;
    }

    if n >= 1_000_000_000_000_000 {
        result += n - 999_999_999_999_999;
    }

    println!("{}", result);
}
