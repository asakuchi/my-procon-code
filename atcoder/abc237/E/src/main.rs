///
/// ダイクストラ（優先度付きキュー）
///
use proconio::fastout;
use proconio::input;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: i128 = 100_100_100_1;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    d: i128,
    v: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.d.cmp(&other.d)
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i128; n],
        uv_org: [(usize, usize); m],
    }

    let uv = uv_org
        .into_iter()
        .map(|(x, y)| (x - 1, y - 1))
        .collect::<Vec<_>>();

    let mut edge = vec![Vec::new(); n];

    for (u, v) in &uv {
        edge[*u].push(*v);
        edge[*v].push(*u);
    }

    // startからの楽しさ
    let mut d = vec![INF; n];
    d[0] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Point { d: 0, v: 0 }));

    while let Some(Reverse(point)) = priority_queue.pop() {
        let now = point.v;
        let nowd = point.d;

        if d[now] != nowd {
            continue;
        }

        for &next in &edge[now] {
            let cost = std::cmp::max(0, h[next] - h[now]);

            let nextd = nowd + cost;

            if nextd >= d[next] {
                continue;
            }
            d[next] = nextd;
            priority_queue.push(Reverse(Point { d: nextd, v: next }));
        }
    }

    let mut answer = 0;

    for i in 0..n {
        let now = h[0] - h[i] - d[i];
        answer = std::cmp::max(answer, now);
    }

    println!("{}", answer);
}
