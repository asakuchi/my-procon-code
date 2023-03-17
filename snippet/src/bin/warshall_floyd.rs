//!
//! 全点対間最短経路
//!
//! ワーシャルフロイド
//!

// use proconio::fastout;
// use proconio::input;

const INF: isize = 1_000_000_000_000_000_000;
// 負の辺がないならusizeで良い
// const INF: usize = 1_000_000_000_000_000_000;

fn main() {
    // 入力値
    let n = 4_usize; // 頂点の数
    let _m = 6_usize; // 辺の数
    let u_v_w = vec![
        (0, 1, 1),
        (0, 2, 5),
        (1, 2, 2),
        (1, 3, 4),
        (2, 3, 1),
        (3, 2, 7),
    ];

    /* --------------------------------------------------- */

    // 始点から各頂点までの最短コスト
    let mut costs = vec![vec![INF; n]; n];

    for i in 0..n {
        for j in 0..n {
            costs[i][j] = if i == j { 0 } else { INF };
        }
    }

    for &(u, v, w) in &u_v_w {
        costs[u][v] = w;
    }

    for k in 0..n {
        for i in 0..n {
            if costs[i][k] == INF {
                continue;
            }

            for j in 0..n {
                if costs[k][j] == INF {
                    continue;
                }

                costs[i][j] = std::cmp::min(costs[i][j], costs[i][k] + costs[k][j]);
            }
        }
    }

    let negative = {
        let mut negative = false;
        for i in 0..n {
            if costs[i][i] < 0 {
                negative = true;
                break;
            }
        }

        negative
    };

    if negative {
        println!("NEGATIVE CYCLE");
        return;
    }

    for i in 0..n {
        for j in 0..n {
            if j != 0 {
                print!(" ");
            }

            if costs[i][j] == INF {
                print!("INF");
            } else {
                print!("{}", costs[i][j]);
            }
        }

        println!();
    }
}
