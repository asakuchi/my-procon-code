use proconio::fastout;
use proconio::input;
use superslice::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut s = vec![Vec::new(); n + 1];

    for i in 0..n {
        s[a[i]].push(i);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        }

        println!("{}", s[x].lower_bound(&r) - s[x].lower_bound(&(l - 1)));
    }
}
