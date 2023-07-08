use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut b = vec![vec!['2'; n]; n];

    for i in 1..n - 1 {
        for j in 1..n - 1 {
            b[i][j] = a[i][j];
        }
    }

    // 左
    for i in 0..n - 1 {
        b[i][0] = a[i + 1][0];
    }

    // 右
    for i in 1..n {
        b[i][n - 1] = a[i - 1][n - 1];
    }

    // 上
    for j in 1..n {
        b[0][j] = a[0][j - 1];
    }

    // 下
    for j in 0..n - 1 {
        b[n - 1][j] = a[n - 1][j + 1];
    }

    for i in 0..n {
        let text = b[i].iter().format("");
        println!("{}", text);
    }
}
