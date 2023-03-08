use proconio::{input, marker::Usize1};
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

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_b_c: [(Usize1, Usize1, usize); m],
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![vec![INF; n]; n];

    for i in 0..n {
        for j in 0..n {
            costs[i][j] = if i == j { 0 } else { INF };
        }
    }

    for &(a, b, c) in &a_b_c {
        costs[a][b] = c;
        costs[b][a] = c;
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

    for load in 0..m {
        let (a, b, c) = a_b_c[load];

        let mut need = false;

        'seach: for i in 0..n {
            for j in 0..n {
                if costs[i][j] == costs[i][a] + costs[b][j] + c {
                    need = true;
                    break 'seach;
                }
            }
        }

        if !need {
            result += 1;
        }
    }

    println!("{}", result);
}
