use proconio::{input, marker::Usize1};

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a_b_c: [(Usize1, Usize1, usize); n-1],
        q: usize,
        k: Usize1,
        x_y: [(Usize1, Usize1); q],
    }

    // ある頂点からの辺と重み
    let mut list = vec![Vec::new(); n];

    for &(a, b, c) in &a_b_c {
        list[a].push((b, c));
        list[b].push((a, c));
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![INF; n];
    costs[k] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Vertex {
        vertex_number: k,
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

    for &(x, y) in &x_y {
        println!("{}", costs[x] + costs[y]);
    }
}
