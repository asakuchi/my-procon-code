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

    // ------- メモ化再帰 -------

    let mut dp = vec![vec![None; MAX_N]; 1 << MAX_N];

    let result = rec(n, &costs, 0, 0, &mut dp);

    println!("{}", result);

    // ------- 漸化式 -------
    // 全て訪問済みのスタートを0にし、
    // そこから逆算してコストを増やしていく
    // 全て未訪問のスタートが答え

    // 全て訪問済みのスタートへ行くまでに必要なコスト
    let mut dp = vec![vec![INF; MAX_N]; 1 << MAX_N];

    // 全て訪問済みのスタート
    dp[(1 << n) - 1][0] = 0;

    for s in (0..=(1 << n) - 2).rev() {
        for v in 0..n {
            for u in 0..n {
                if s >> u & 1 > 0 {
                    continue;
                }

                dp[s][v] = dp[s][v].min(dp[s | 1 << u][u] + costs[v][u]);
            }
        }
    }

    // 全て未訪問のスタート
    println!("{}", dp[0][0]);

    // ------- 漸化式 -------
    // ゴール頂点を追加する
    // スタートのみ訪問済みのスタートから重みを加算していく
    // 全て訪問済みのゴールが答え

    // スタートのみ訪問済みのスタートからの重みの合計
    let mut dp = vec![vec![INF; MAX_N]; 1 << MAX_N];

    let mut costs = vec![vec![0; n + 1]; n + 1];

    // 入力値を修正
    for i in 0..n {
        for j in 0..n {
            if j == 0 {
                // スタートへ向かう有向辺をINFにし、
                // 代わりにゴールへの有向辺を追加する
                costs[i][0] = INF;

                if d[i][j] == -1 {
                    costs[i][n] = INF;
                } else {
                    costs[i][n] = d[i][j] as usize;
                }
            } else {
                if d[i][j] == -1 {
                    costs[i][j] = INF;
                } else {
                    costs[i][j] = d[i][j] as usize;
                }
            }
        }
    }

    // ゴール地点を追加
    let n = n + 1;

    // スタートのみ訪問済みのスタート
    dp[1 << 0][0] = 0;

    for s in 0..1 << n {
        for v in 0..n {
            for u in 0..n {
                dp[s | 1 << u][u] = dp[s | 1 << u][u].min(dp[s][v] + costs[v][u]);
            }
        }
    }

    // 全て訪問済みのゴール
    println!("{}", dp[(1 << n) - 1][n - 1]);
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
