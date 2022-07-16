use num_integer::*;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", a.gcd(&b));
}
