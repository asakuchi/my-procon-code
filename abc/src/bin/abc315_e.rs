use std::collections::{HashSet, VecDeque};

use ac_library::SccGraph;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        c: [[Usize1]; n],
    }

    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited.insert(0);

    while let Some(current) = queue.pop_front() {
        for &next in c[current].iter() {
            if visited.contains(&next) {
                continue;
            }

            visited.insert(next);
            queue.push_back(next);
        }
    }

    // -----------------------------------------------

    let mut g = SccGraph::new(n);

    for i in 0..n {
        for &next in c[i].iter() {
            if !visited.contains(&next) {
                continue;
            }

            g.add_edge(i, next);
        }
    }

    let scc = g.scc();

    let mut result = Vec::new();

    for com in scc.iter() {
        for &v in com {
            if visited.contains(&v) && v != 0 {
                result.push(v);
            }
        }
    }

    let text = result.iter().rev().map(|v| v + 1).format(" ");

    println!("{}", text);
}
