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
    input! {
        n: usize,
        m: usize,
        mut a_b_t: [(usize, usize,usize); m],
    }

    /* --------------------------------------------------- */

    // 始点から各頂点までの最短コスト
    let mut costs = vec![vec![INF; n]; n];

    for i in 0..n {
        for j in 0..n {
            costs[i][j] = if i == j { 0 } else { INF };
        }
    }

    for (s, t, d) in &a_b_t {
        costs[*s - 1][*t - 1] = *d;
        costs[*t - 1][*s - 1] = *d;
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

    let mut result = std::usize::MAX;

    for i in 0..n {
        let mut station_result = 0;
        for j in 0..n {
            station_result = std::cmp::max(station_result, costs[i][j]);
        }

        result = std::cmp::min(result, station_result);
    }

    println!("{}", result);
}
