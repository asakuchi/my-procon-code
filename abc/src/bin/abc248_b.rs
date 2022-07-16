use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    let mut result = 0;

    let mut num = a;

    while num < b {
        num *= k;
        result += 1;
    }

    println!("{}", result);
}
