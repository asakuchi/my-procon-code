use std::collections::VecDeque;

use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: usize,
        p: usize,
        q: usize,
        c: [Usize1; k],
        a_b: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    let mut dangerous = vec![false; n];
    let mut zombie = vec![false; n];

    let mut queue = VecDeque::new();

    for &town in &c {
        queue.push_back((town, 0));
        dangerous[town] = true;

        zombie[town] = true;
    }

    while let Some((current, step)) = queue.pop_front() {
        if step == s {
            continue;
        }

        for &next in list[current].iter() {
            if dangerous[next] {
                continue;
            }

            // 次へ
            dangerous[next] = true;
            queue.push_back((next, step + 1));
        }
    }

    let mut list = Vec::new();

    for &(a, b) in &a_b {
        if zombie[a] || zombie[b] {
            continue;
        }

        list.push((
            a,
            b,
            if b == n - 1 {
                0
            } else if dangerous[b] {
                q
            } else {
                p
            },
        ));
        list.push((
            b,
            a,
            if a == n - 1 {
                0
            } else if dangerous[a] {
                q
            } else {
                p
            },
        ));
    }

    let graph = DiGraph::<(), usize, usize>::from_edges(&list);

    let res = dijkstra(&graph, 0.into(), Some((n - 1).into()), |e| *e.weight());

    println!("{}", res.get(&(n - 1).into()).unwrap());
}
