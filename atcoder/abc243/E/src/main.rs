use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(Usize1, Usize1, isize); m],
    }

    // 始点から各頂点までの最短コスト
    let mut costs_all = vec![vec![INF; n]; n];

    for i in 0..n {
        for j in 0..n {
            costs_all[i][j] = if i == j { 0 } else { INF };
        }
    }

    for (s, t, d) in &abc {
        costs_all[*s][*t] = *d;
        costs_all[*t][*s] = *d;
    }

    for k in 0..n {
        for i in 0..n {
            if costs_all[i][k] == INF {
                continue;
            }

            for j in 0..n {
                if costs_all[k][j] == INF {
                    continue;
                }

                costs_all[i][j] = std::cmp::min(costs_all[i][j], costs_all[i][k] + costs_all[k][j]);
            }
        }
    }

    let mut result = 0;

    for &(s, t, d) in &abc {
        let mut unused = 0;

        for i in 0..n {
            if i == s || i == t {
                continue;
            }
            if costs_all[s][i] + costs_all[i][t] <= d {
                unused = 1;
            }
        }

        result += unused;
    }

    println!("{}", result);
}
