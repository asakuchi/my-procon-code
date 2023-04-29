//!
//! しゃくとり法
//!

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut result = 0;
    let mut s = 0;
    let mut t = 0;

    let mut sum = a[0];

    loop {
        while sum < k && t < n - 1 {
            t += 1;
            sum += a[t];
        }

        while sum >= k {
            result += n - t;

            sum -= a[s];
            s += 1;
        }

        if t == n - 1 {
            break;
        }
    }

    println!("{}", result);
}
