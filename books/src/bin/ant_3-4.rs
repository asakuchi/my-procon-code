use proconio::input;

const MAX_N: usize = 18;
const INF: usize = 1_000 * 20; // 十分大きい数

///
/// ビットDP
/// 巡回セールスマン問題
///
fn main() {
    // 入力
    // 5
    // -1 3 -1 4 -1
    // -1 -1 5 -1 -1
    // 4 -1 -1 5 -1
    // -1 -1 -1 -1 3
    // 7 6 -1 -1 -1
    //
    // 出力
    // 22

    input! {
        n: usize,
        mut d: [[isize; n]; n],
    }

    let mut costs = vec![vec![0; n]; n];

    // 入力値を修正
    for i in 0..n {
        for j in 0..n {
            if d[i][j] == -1 {
                costs[i][j] = INF;
            } else {
                costs[i][j] = d[i][j] as usize;
            }
        }
    }

    let mut dp = vec![vec![None; MAX_N]; 1 << MAX_N];

    let result = rec(n, &costs, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    costs: &Vec<Vec<usize>>,
    visited: usize,
    current: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if let Some(value) = dp[visited][current] {
        return value;
    }

    if visited == (1 << n) - 1 && current == 0 {
        // 探索完了
        dp[visited][current] = Some(0);
        return 0;
    }

    let mut result = INF;

    for next in 0..n {
        if visited & 1 << next == 0 {
            let u_score = rec(n, costs, visited | 1 << next, next, dp) + costs[current][next];

            result = std::cmp::min(result, u_score);
        }
    }

    dp[visited][current] = Some(result);

    result
}
