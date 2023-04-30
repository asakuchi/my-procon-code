use proconio::fastout;
use proconio::{input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        l_r: [(usize, usize); q],
    }

    let mut accum = vec![0; n + 1];

    for i in 1..n {
        accum[i + 1] = accum[i];

        if s[i - 1] == 'A' && s[i] == 'C' {
            accum[i + 1] += 1;
        }
    }

    for &(l, r) in &l_r {
        let result = accum[r] - accum[l];

        println!("{}", result);
    }
}
