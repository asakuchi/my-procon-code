use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: usize = 1_000_000_100_100_100_1;

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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    // 入力値
    let v = n; // 頂点の数
    let _e = m; // 辺の数
    let r = 0; // 始点
    let s_t_d = abc.clone();

    /* --------------------------------------------------- */

    // ある頂点からの辺と重み
    let mut edge = vec![Vec::new(); v];

    for (s, t, d) in &s_t_d {
        edge[*s].push((*t, *d));
        edge[*t].push((*s, *d));
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![INF; v];
    costs[r] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Vertex {
        vertex_number: r,
        weight: 0,
    }));

    let mut used = vec![-1; n];

    while let Some(Reverse(point)) = priority_queue.pop() {
        let now = point.vertex_number;
        let nowd = point.weight;

        if costs[now] < nowd {
            continue;
        }

        for (edge_v, edge_d) in &edge[now] {
            let nextd = nowd + *edge_d;

            if nextd >= costs[*edge_v] {
                continue;
            }

            used[*edge_v] = now as isize;

            costs[*edge_v] = nextd;

            priority_queue.push(Reverse(Vertex {
                vertex_number: *edge_v,
                weight: nextd,
            }));
        }
    }

    let mut load_index_map = std::collections::HashMap::new();

    for i in 0..m {
        let (a, b, _c) = abc[i];

        load_index_map.insert((a, b), i);
        load_index_map.insert((b, a), i);
    }

    for i in 1..n {
        if let Some(load_index) = load_index_map.get(&(used[i] as usize, i)) {
            println!("{}", load_index + 1);
        } else {
            panic!("load not found");
        }
    }
}
