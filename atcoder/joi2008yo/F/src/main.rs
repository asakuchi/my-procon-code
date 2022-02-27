use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut matrix = vec![vec![INF; n]; n];

    for _ in 0..k {
        input! {record_type:usize}

        match record_type {
            // 注文票
            0 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }

                // 始点から各頂点までの最短コスト
                let mut costs = vec![INF; n];
                costs[a] = 0;

                let mut priority_queue = BinaryHeap::new();
                priority_queue.push(Reverse(Vertex {
                    vertex_number: a,
                    weight: 0,
                }));

                while let Some(Reverse(point)) = priority_queue.pop() {
                    let now = point.vertex_number;
                    let nowd = point.weight;

                    if costs[now] != nowd {
                        continue;
                    }

                    for next in 0..n {
                        let edge_v = next;
                        let edge_d = matrix[now][next];

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

                if costs[b] == INF {
                    println!("-1");
                } else {
                    println!("{}", costs[b]);
                }
            }
            // 運行情報
            _ => {
                input! {
                    c: Usize1,
                    d: Usize1,
                    e: usize,
                }

                let new_cost = std::cmp::min(e, matrix[c][d]);

                matrix[c][d] = new_cost;
                matrix[d][c] = new_cost;
            }
        }
    }
}

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: usize = 100_100_100_1;

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
