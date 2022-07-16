use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    let mut result: i64 = 1_000_000_000_000_000_000;
    let mut j = 1_000_000;

    for i in 0..=1_000_000 {
        while f(i, j) >= n && j >= 0 {
            result = result.min(f(i, j));
            j -= 1;
        }
    }

    println!("{}", result);
}

fn f(a: i64, b: i64) -> i64 {
    a * a * a + a * a * b + a * b * b + b * b * b
}
