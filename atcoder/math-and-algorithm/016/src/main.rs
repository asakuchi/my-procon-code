use num_integer::*;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = a[0];

    for num in a {
        result = result.gcd(&num);
    }

    println!("{}", result);
}
