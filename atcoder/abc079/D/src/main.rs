use proconio::fastout;
use proconio::input;
use std::cmp::Ordering;

const INF: usize = 100_100_100_100;

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
#[fastout]
fn main() {
    let n = 10;

    input! {
        h:usize,
        w:usize,
        c: [[usize;n];n],
        a: [[isize;w];h],
    }

    /* --------------------------------------------------- */

    // 始点から各頂点までの最短コスト
    let mut costs = vec![vec![INF; n]; n];

    for i in 0..n {
        for j in 0..n {
            costs[i][j] = c[i][j];
        }
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

    let mut result = 0;

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == -1 {
                continue;
            }

            result += costs[a[i][j] as usize][1];
        }
    }

    println!("{}", result);
}
