use itertools::Itertools;
use proconio::{input, marker::Usize1};

use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
        k: usize,
        a_b: [(Usize1, Usize1); m],
        p_h: [(Usize1, usize); k],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![None; n];

    let mut priority_queue = BinaryHeap::new();

    for &(p, h) in &p_h {
        costs[p] = Some(h);
        // visited[p] = true;

        // queue.push_back((p, h));
        priority_queue.push(Vertex {
            vertex_number: p,
            weight: h,
        })
    }

    while let Some(point) = priority_queue.pop() {
        let current = point.vertex_number;
        let current_cost = point.weight;

        if current_cost == 0 {
            continue;
        }

        if let Some(value) = costs[current] {
            if value != current_cost {
                continue;
            }
        }

        for &next in &list[current] {
            let next_cost = current_cost - 1;

            if let Some(value) = costs[next] {
                if next_cost <= value {
                    continue;
                }
            }

            costs[next] = Some(next_cost);

            priority_queue.push(Vertex {
                vertex_number: next,
                weight: next_cost,
            });
        }
    }

    let mut result = Vec::new();

    for i in 0..n {
        if let Some(_) = costs[i] {
            result.push(i + 1);
        }
    }

    println!("{}", result.len());

    let text = result.iter().format(" ");
    println!("{}", text);
}
