//!
//! 拡張ダイクストラ
//!

use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: usize = 1_100_100_100_100_100_101;

#[derive(Debug, Eq, PartialEq)]
struct Vertex {
    /// 重み
    weight: usize,
    /// 頂点番号,所持銀貨
    vertex_number: (usize, usize),
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

const SILVER: usize = 3_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        u_v_a_b: [(Usize1, Usize1, usize, usize); m],
        c_d: [(usize, usize); n],
    }

    // ある頂点からの辺と重み
    let mut edge = vec![vec![Vec::new(); SILVER]; n];

    for i in 0..n {
        let (c, d) = c_d[i];

        for j in 0..SILVER - c {
            edge[i][j].push(((i, j + c), d));
        }
    }

    for &(u, v, a, b) in &u_v_a_b {
        for coin in a..SILVER {
            edge[u][coin].push(((v, coin - a), b));
            edge[v][coin].push(((u, coin - a), b));
        }
    }

    // 全頂点を回ることができる十分な銀貨より多く持つ必要はないので捨てる
    let s = s.min(SILVER - 1);

    // 始点から各頂点までの最短コスト
    let mut costs = vec![vec![INF; SILVER]; n];
    // (頂点、所持銀貨)
    costs[0][s] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Vertex {
        vertex_number: (0, s),
        weight: 0,
    }));

    while let Some(Reverse(point)) = priority_queue.pop() {
        let now = point.vertex_number;
        let nowd = point.weight;

        if costs[now.0][now.1] != nowd {
            continue;
        }

        for (edge_v, edge_d) in &edge[now.0][now.1] {
            let nextd = nowd + *edge_d;

            if nextd >= costs[edge_v.0][edge_v.1] {
                continue;
            }

            costs[edge_v.0][edge_v.1] = nextd;
            priority_queue.push(Reverse(Vertex {
                vertex_number: *edge_v,
                weight: nextd,
            }));
        }
    }

    for i in 1..n {
        let mut min = INF;

        for j in 0..SILVER {
            min = min.min(costs[i][j]);
        }

        println!("{}", min);
    }
}
