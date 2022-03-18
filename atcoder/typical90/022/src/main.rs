use num::integer::Integer;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let divisor = a.gcd(&b.gcd(&c));

    println!(
        "{}",
        (a / divisor - 1) + (b / divisor - 1) + (c / divisor - 1)
    );
}
