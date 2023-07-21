// 解説AC

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

fn main() {
    let (n, m, x) = input_tuple();
    let u_v_a_b = input_tuple_vec::<usize>(m);

    let mut ok: isize = 0;
    let mut ng = 1 << 60;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let solve = || {
            // ある頂点からの辺と重み
            let mut list = vec![Vec::new(); n];

            for &(u, v, a, b) in &u_v_a_b {
                if b as isize >= mid {
                    list[u - 1].push((v - 1, a));
                    list[v - 1].push((u - 1, a));
                }
            }

            let start = 0;

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

            costs[n - 1] <= x
        };

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    if ok != 0 {
        println!("{}", ok);
    } else {
        println!("-1");
    }
}

fn input_tuple<T>() -> (T, T, T)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();
    let l = iter.next().unwrap().parse().unwrap();

    (n, m, l)
}

fn input_tuple_vec<T>(n: usize) -> Vec<(T, T, T, T)>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // タプルのベクタ

    let stdin = std::io::stdin();

    let mut s_t_d = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s = iter.next().unwrap().parse().unwrap();
        let t = iter.next().unwrap().parse().unwrap();
        let d = iter.next().unwrap().parse().unwrap();
        let x = iter.next().unwrap().parse().unwrap();

        s_t_d.push((s, t, d, x));
    }

    s_t_d
}
