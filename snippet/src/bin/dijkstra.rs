//!
//! AOJ では自作コードを使う
//!

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
// use std::io;

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

enum Directed {
    Directed,
    Undirected,
}

///
/// 単一始点最短経路
/// ダイクストラ
///
fn main() {
    // 入力値
    let v = 4; // 頂点の数
    let _e = 5; // 辺の数
    let r = 0; // 始点
    let s_t_d = vec![(0, 1, 1), (0, 2, 4), (1, 2, 2), (2, 3, 1), (1, 3, 5)];
    let direction = Directed::Directed;
    /* --------------------------------------------------- */

    // ある頂点からの辺と重み
    let mut edge = vec![Vec::new(); v];

    for (s, t, d) in &s_t_d {
        edge[*s].push((*t, *d));

        if let direction = Directed::Undirected {
            edge[*t].push((*s, *d));
        }
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![INF; v];
    costs[r] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Vertex {
        vertex_number: r,
        weight: 0,
    }));

    while let Some(Reverse(point)) = priority_queue.pop() {
        let now = point.vertex_number;
        let nowd = point.weight;

        if costs[now] != nowd {
            continue;
        }

        for (edge_v, edge_d) in &edge[now] {
            let nextd = nowd + *edge_d;

            if nextd >= costs[*edge_v] {
                continue;
            }
            costs[*edge_v] = nextd;
            priority_queue.push(Reverse(Vertex {
                vertex_number: *edge_v,
                weight: nextd,
            }));
        }
    }

    for i in 0..v {
        if costs[i] == INF {
            println!("INF");
        } else {
            println!("{}", costs[i]);
        }
    }
}
