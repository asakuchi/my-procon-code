// -*- coding:utf-8-unix -*-

use itertools::Itertools;
use proconio::input;

///
/// 解説見た後のコード
///
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m],
    }

    // ボールの対応表を作成
    let mut ab_matrix = vec![vec![false; n]; n];
    let mut cd_matrix = vec![vec![false; n]; n];

    for (a, b) in ab.iter() {
        ab_matrix[*a - 1][*b - 1] = true;
        ab_matrix[*b - 1][*a - 1] = true;
    }

    for (c, d) in cd.iter() {
        cd_matrix[*c - 1][*d - 1] = true;
        cd_matrix[*d - 1][*c - 1] = true;
    }

    let perms = (0..n).permutations(n);

    for p in perms {
        let mut ok = true;

        for i in 0..n {
            for j in 0..n {
                if ab_matrix[i][j] != cd_matrix[p[i]][p[j]] {
                    ok = false;
                }
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
