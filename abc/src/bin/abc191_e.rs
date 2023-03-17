use proconio::{input, marker::Usize1};

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
        n: usize,
        m: usize,
        a_b_c: [(Usize1, Usize1, usize); m],
    }

    // ある頂点からの辺と重み
    let mut list = vec![Vec::new(); n * 2];

    for &(a, b, c) in &a_b_c {
        list[a].push((b, c));

        // 出発点と到着点を別扱いにする
        list[a].push((b + n, c));
    }

    for start in 0..n {
        // 始点から各頂点までの最短コスト
        let mut costs = vec![INF; 2 * n];
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

        if costs[start + n] == INF {
            println!("-1");
        } else {
            println!("{}", costs[start + n]);
        }
    }
}
