use proconio::{input, marker::Usize1};
use std::cmp::Ordering;

const INF: usize = 1_100_100_100_100_100_101;

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

fn main() {
    input! {
        v: usize,
        m: usize,
        s_t_d: [(Usize1, Usize1, usize); m],
    }

    let mut result = 0;

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

        for i in 0..v {
            for j in 0..v {
                if costs[i][j] != INF {
                    result += costs[i][j];
                }
            }
        }
    }

    println!("{}", result);
}
