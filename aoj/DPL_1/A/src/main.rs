use std::io;

fn main() {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let mut c: Vec<usize> = Vec::with_capacity(m);

    for ci in iter {
        c.push(ci.parse().unwrap());
    }

    /* --------------------------------------------------- */

    // let mut dp = vec![vec![0; n + 1]; m + 1];

    // for i in 0..m {
    //     for j in 1..=n {
    //         if vw[i].1 > j {
    //             dp[i + 1][j] = dp[i][j];
    //         } else {
    //             dp[i + 1][j] = std::cmp::max(dp[i][j], dp[i + 1][j - vw[i].1] + vw[i].0);
    //         }
    //     }
    // }

    // println!("{}", dp[n][w]);

    let mut dp = vec![vec![-1; n + 1]; m + 1];
    let result = rec(n, m, &c, 0, n as isize, &mut dp);

    println!("{}", result);
}

fn rec(n: usize, m: usize, c: &Vec<usize>, i: usize, j: isize, dp: &mut Vec<Vec<isize>>) -> usize {
    if j == 0 {
        return 0;
    }

    if i >= m || j < 0 {
        return 100_000_000;
    }

    if dp[i][j as usize] != -1 {
        return dp[i][j as usize] as usize;
    }

    // 使わない
    let score_1 = rec(n, m, c, i + 1, j, dp);
    // 使う
    let score_2 = rec(n, m, c, i, j - c[i] as isize, dp) + 1;

    let score = std::cmp::min(score_1, score_2);

    dp[i][j as usize] = score as isize;

    score
}
