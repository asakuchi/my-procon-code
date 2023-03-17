//!
//! 単一始点最短経路
//!
//! ダイクストラ
//!
//! AOJ では自作コードを使う
//!

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
// use std::io;

const INF: usize = 1_000_000_000_000_000_000;

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
    // 入力値
    let n = 4; // 頂点の数
    let _m = 5; // 辺の数
    let start = 0; // 始点
    let u_v_w = vec![(0, 1, 1), (0, 2, 4), (1, 2, 2), (2, 3, 1), (1, 3, 5)];
    /* --------------------------------------------------- */

    // ある頂点からの辺と重み
    let mut list = vec![Vec::new(); n];

    for &(u, v, w) in &u_v_w {
        list[u].push((v, w));

        // 無向グラフの場合
        // list[t].push((s, d));
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![INF; n];
    costs[start] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Vertex {
        vertex_number: start,
        weight: 0,
    }));

    while let Some(Reverse(point)) = priority_queue.pop() {
        let current = point.vertex_number;
        let current_cost = point.weight;

        if costs[current] != current_cost {
            continue;
        }

        for &(next, edge_weight) in &list[current] {
            let next_cost = current_cost + edge_weight;

            if next_cost >= costs[next] {
                continue;
            }

            costs[next] = next_cost;

            priority_queue.push(Reverse(Vertex {
                vertex_number: next,
                weight: next_cost,
            }));
        }
    }

    for i in 0..n {
        if costs[i] == INF {
            println!("INF");
        } else {
            println!("{}", costs[i]);
        }
    }
}
