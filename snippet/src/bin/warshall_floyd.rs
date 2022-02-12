use proconio::fastout;
use proconio::input;
use std::cmp::Ordering;

const INF: isize = 100_100_100_100;

#[derive(Debug, Eq, PartialEq)]
struct Vertex {
    /// 重み
    weight: usize,
    /// 頂点番号
    vertex_number: usize,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl std::cmp::PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

///
/// 全点対間最短経路
/// ワーシャルフロイド
///
fn main() {
    // 入力値
    let v = 4; // 頂点の数
    let e = 6; // 辺の数
    let s_t_d = vec![
        (0, 1, 1),
        (0, 2, 5),
        (1, 2, 2),
        (1, 3, 4),
        (2, 3, 1),
        (3, 2, 7),
    ];

    /* --------------------------------------------------- */

    // 始点から各頂点までの最短コスト
    let mut costs = vec![vec![INF; v]; v];

    for i in 0..v {
        for j in 0..v {
            costs[i][j] = if i == j { 0 } else { INF };
        }
    }

    for (s, t, d) in &s_t_d {
        costs[*s][*t] = *d;
    }

    for k in 0..v {
        for i in 0..v {
            if costs[i][k] == INF {
                continue;
            }

            for j in 0..v {
                if costs[k][j] == INF {
                    continue;
                }

                costs[i][j] = std::cmp::min(costs[i][j], costs[i][k] + costs[k][j]);
            }
        }
    }

    let negative = {
        let mut negative = false;
        for i in 0..v {
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

    for i in 0..v {
        for j in 0..v {
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
