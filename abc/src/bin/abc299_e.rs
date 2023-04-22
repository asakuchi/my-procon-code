use itertools::Itertools;
use proconio::{input, marker::Usize1};
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
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
        k: usize,
        p_d: [(Usize1, usize); k],
    }

    if k == 0 {
        println!("Yes");
        println!("1{}", "0".repeat(n - 1));
        return;
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    let mut requirement = vec![None; n];

    for &(p, d) in &p_d {
        requirement[p] = Some(d);
    }

    let mut result = vec![1; n];

    for &(p, d) in &p_d {
        // 始点から各頂点までの最短コスト
        let mut costs = vec![INF; n];
        costs[p] = 0;

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(Reverse(Vertex {
            vertex_number: p,
            weight: 0,
        }));

        while let Some(Reverse(point)) = priority_queue.pop() {
            let current = point.vertex_number;
            let current_cost = point.weight;

            if costs[current] != current_cost {
                continue;
            }

            // 探しているコスト
            if costs[current] < d {
                // d 以内に黒がいてはいけない
                result[current] = 0;
            }

            // for &(next, edge_weight) in &list[current] {
            for &next in &list[current] {
                // 重みは常に1とする
                let edge_weight = 1;

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
    }

    for &(p, d) in &p_d {
        // 始点から各頂点までの最短コスト
        let mut costs = vec![INF; n];
        costs[p] = 0;

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(Reverse(Vertex {
            vertex_number: p,
            weight: 0,
        }));

        let mut ok = false;

        while let Some(Reverse(point)) = priority_queue.pop() {
            let current = point.vertex_number;
            let current_cost = point.weight;

            if costs[current] != current_cost {
                continue;
            }

            // 探しているコスト
            if costs[current] == d {
                if result[current] == 1 {
                    ok = true;
                }
            }

            // for &(next, edge_weight) in &list[current] {
            for &next in &list[current] {
                // 重みは常に1とする
                let edge_weight = 1;

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

        if !ok {
            // 塗る方法がない
            println!("No");
            return;
        }
    }

    // println!("Yes");

    let text = result.iter().format("");

    let text = text.to_string();

    println!("Yes");
    println!("{}", text);
}
