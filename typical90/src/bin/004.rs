// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 004 - Cross Sum（★2）
///
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],  // Vec<(i32, i32, i32)>
    }

    let mut sum_low = vec![0; h];
    let mut sum_column = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            sum_low[i] += a[i][j];
            sum_column[j] += a[i][j];
        }
    }

    // println!("{:?}", a);
    // println!("{:?}", sum_low);
    // println!("{:?}", sum_column);

    let mut result = vec![vec![0; w]; h];

    // println!("{:?}", result);

    for i in 0..h {
        for j in 0..w {
            result[i][j] = sum_low[i] + sum_column[j] - a[i][j]; // a[i][j] がダブルので1個減らす
        }
    }

    for line in result {
        println!(
            "{}",
            line.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
