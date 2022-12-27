//!
//! 二次元累積和
//!

use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        m: usize,
        n: usize,
        k: usize,
        map: [Chars; m],
        area: [(usize, usize, usize, usize); k],
    }

    let mut s = vec![vec![vec![0; n + 1]; m + 1]; 3];

    for i in 0..m {
        for j in 0..n {
            s[match map[i][j] {
                'J' => 0,
                'O' => 1,
                _ => 2,
            }][i + 1][j + 1] += 1;

            for l in 0..3 {
                s[l][i + 1][j + 1] += s[l][i + 1][j] + s[l][i][j + 1] - s[l][i][j];
            }
        }
    }

    for (a, b, c, d) in area {
        let mut line = Vec::new();

        for i in 0..3 {
            line.push(s[i][c][d] + s[i][a - 1][b - 1] - s[i][a - 1][d] - s[i][c][b - 1]);
        }

        println!("{}", line.into_iter().join(" "));
    }
}
