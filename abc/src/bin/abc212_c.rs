use proconio::fastout;
use proconio::input;
use superslice::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [isize; n],
        mut b: [isize; m],
    }

    a.sort();
    b.sort();

    let mut result = 2_000_000_000;

    for i in 0..n {
        let j = b.lower_bound(&a[i]);

        if j < m {
            result = result.min((a[i] - b[j]).abs());
        }
        if j > 0 {
            result = result.min((a[i] - b[j - 1]).abs());
        }
    }

    println!("{}", result);
}
