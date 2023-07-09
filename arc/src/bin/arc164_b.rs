use std::collections::VecDeque;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
        c: [usize; n],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    let mut uf = UnionFind::new(n);

    // UF BFS

    let mut visited = vec![false; n];

    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;

    while let Some(current) = queue.pop_front() {
        for &next in list[current].iter() {
            if c[current] != c[next] {
                uf.union(current, next);
            }

            if visited[next] {
                continue;
            }

            visited[next] = true;
            queue.push_back(next);
        }
    }

    // 検知 BFS

    let mut visited = vec![false; n];

    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;

    while let Some(current) = queue.pop_front() {
        for &next in list[current].iter() {
            if c[current] == c[next] && uf.equiv(current, next) {
                println!("Yes");
                return;
            }

            if visited[next] {
                continue;
            }

            visited[next] = true;
            queue.push_back(next);
        }
    }

    println!("No");
}
