use std::cmp::Ordering;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

const INF: isize = 100_100_100_100_100_100;

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
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        u_v: [(Usize1, Usize1); q],
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![vec![(INF, 0); n]; n];

    for i in 0..n {
        for j in 0..n {
            costs[i][j] = if i == j { (0, 0) } else { (INF, 0) };
        }
    }

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                costs[i][j] = (1, a[j]);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            if costs[i][k].0 == INF {
                continue;
            }

            for j in 0..n {
                if costs[k][j].0 == INF {
                    continue;
                }

                if costs[i][k].0 + costs[k][j].0 < costs[i][j].0 {
                    costs[i][j] = (costs[i][k].0 + costs[k][j].0, costs[i][k].1 + costs[k][j].1);
                } else if costs[i][k].0 + costs[k][j].0 == costs[i][j].0 {
                    if costs[i][k].1 + costs[k][j].1 > costs[i][j].1 {
                        costs[i][j] =
                            (costs[i][k].0 + costs[k][j].0, costs[i][k].1 + costs[k][j].1);
                    }
                }
            }
        }
    }

    for (u, v) in u_v {
        if costs[u][v].0 == INF {
            println!("Impossible");
        } else {
            println!("{} {}", costs[u][v].0, costs[u][v].1 + a[u]);
        }
    }
}
