//!
//! 区間DP
//!
use std::io;

const INF: usize = 1_000_000_000_000;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    // ------------------------------------
    // タプルのベクタ

    let mut r_c: Vec<_> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let r: usize = iter.next().unwrap().parse().unwrap();
        let c: usize = iter.next().unwrap().parse().unwrap();

        r_c.push((r, c));
    }

    // ------------------------------------

    if n == 1 {
        println!("0");
        return;
    }

    let mut dp = vec![vec![(INF, (0, 0)); n + 1]; n + 1];

    let result = rec(n, &r_c, 0, n, &mut dp);

    println!("{}", result.0);
}

fn rec(
    n: usize,
    r_c: &Vec<(usize, usize)>,
    left: usize,
    right: usize,
    dp: &mut Vec<Vec<(usize, (usize, usize))>>,
) -> (usize, (usize, usize)) {
    if left + 1 == right {
        return (0, r_c[left]);
    }

    if dp[left][right].0 != INF {
        return dp[left][right];
    }

    let mut result = (INF, (0, 0));

    for i in left + 1..right {
        let (left_count, (left_r, left_c)) = rec(n, r_c, left, i, dp);
        let (right_count, (_right_r, right_c)) = rec(n, r_c, i, right, dp);

        let score = left_count + right_count + left_r * left_c * right_c;
        let new_matrix = (left_r, right_c);

        if score < result.0 {
            result = (score, new_matrix);
        }
    }

    dp[left][right] = result;

    result
}
