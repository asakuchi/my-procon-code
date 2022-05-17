use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        l: u128,
    }

    let n = l - 1;

    let mut result = 1;

    for i in n - 10..=n {
        result *= i;
    }

    for i in 1..=11 {
        result /= i;
    }

    println!("{}", result);
}
