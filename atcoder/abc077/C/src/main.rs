use proconio::fastout;
use proconio::input;
use superslice::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    a.sort();
    b.sort();
    c.sort();

    let mut result = 0;

    for bi in &b {
        let ai = a.lower_bound(bi);
        let ci = c.upper_bound(bi);

        result += ai * (c.len() - ci);
    }

    println!("{}", result);
}
