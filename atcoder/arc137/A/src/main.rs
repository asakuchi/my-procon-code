use num_integer::*;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let mut result = 0;

    for i in 0..=1500.min(l) {
        for j in 0..=(1500.min(r)) {
            let x = l + i;
            let y = r - j;

            if x > y {
                continue;
            }

            if y.gcd(&x) == 1 {
                result = result.max(y - x);
            }
        }
    }

    println!("{}", result);
}
