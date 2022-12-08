use std::collections::VecDeque;

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Usize1;

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
        k: usize,
        c_r: [(usize, usize); n],
        a_b: [(Usize1, Usize1); k],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    let mut list_cost = vec![Vec::new(); n];

    for start in 0..n {
        let mut queue = VecDeque::new();

        let mut visited = vec![false; n];

        queue.push_back((start, 0));
        visited[start] = true;

        let (c, r) = c_r[start];

        while let Some((current, step)) = queue.pop_front() {
            if step == r {
                continue;
            }

            for &next in list[current].iter() {
                if visited[next] {
                    continue;
                }

                // 次へ
                visited[next] = true;
                queue.push_back((next, step + 1));

                list_cost[start].push((next, c));
            }
        }
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![INF; n];
    costs[0] = 0;

    let mut priority_queue = BinaryHeap::new();

    priority_queue.push(Reverse(Vertex {
        vertex_number: 0,
        weight: 0,
    }));

    while let Some(Reverse(point)) = priority_queue.pop() {
        let now = point.vertex_number;
        let nowd = point.weight;

        if costs[now] != nowd {
            continue;
        }

        for &(edge_v, edge_d) in &list_cost[now] {
            let nextd = nowd + edge_d;

            if nextd >= costs[edge_v] {
                continue;
            }

            costs[edge_v] = nextd;

            priority_queue.push(Reverse(Vertex {
                vertex_number: edge_v,
                weight: nextd,
            }));
        }
    }

    println!("{}", costs[n - 1]);
}
