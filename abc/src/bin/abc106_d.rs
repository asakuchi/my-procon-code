//!
//! 二次元累積和
//!

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        l_r: [(usize, usize); m],
        p_q: [(usize, usize); q],
    }

    let mut s = vec![vec![0; n + 2]; n + 2];

    for &(l, r) in &l_r {
        s[l][r] += 1;
    }

    for i in 0..=n {
        for j in 0..=n {
            s[i + 1][j + 1] += s[i + 1][j] + s[i][j + 1] - s[i][j];
        }
    }

    for &(p, q) in &p_q {
        let result = s[q][q] + s[p - 1][p - 1] - s[p - 1][q] - s[q][p - 1];

        println!("{}", result);
    }
}
