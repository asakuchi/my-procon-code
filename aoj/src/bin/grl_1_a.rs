use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

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

///
/// 単一始点最短経路
/// ダイクストラ
///
fn main() {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let v: usize = iter.next().unwrap().parse().unwrap();
    let e: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();

    let mut s_t_d: Vec<(usize, usize, usize)> = Vec::with_capacity(e);

    for _ in 0..e {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s: usize = iter.next().unwrap().parse().unwrap();
        let t: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();

        s_t_d.push((s, t, d));
    }

    /* --------------------------------------------------- */

    // ある頂点からの辺と重み
    let mut edge = vec![Vec::new(); v];

    for (s, t, d) in &s_t_d {
        edge[*s].push((*t, *d));
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
