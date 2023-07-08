use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n_1: usize,
        n_2: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let n = n_1 + n_2;

    // ある頂点からの辺と重み
    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    let mut steps = vec![0; n];

    let mut visited = vec![false; n];

    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;

    while let Some(current) = queue.pop_front() {
        for &next in list[current].iter() {
            if visited[next] {
                continue;
            }

            visited[next] = true;
            steps[next] = steps[current] + 1;
            queue.push_back(next);
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(n - 1);
    visited[n - 1] = true;

    while let Some(current) = queue.pop_front() {
        for &next in list[current].iter() {
            if visited[next] {
                continue;
            }

            visited[next] = true;
            steps[next] = steps[current] + 1;
            queue.push_back(next);
        }
    }

    let mut max_1 = 0;
    let mut max_2 = 0;

    for i in 0..n_1 {
        max_1 = max_1.max(steps[i]);
    }

    for i in n_1..n {
        max_2 = max_2.max(steps[i]);
    }

    println!("{}", max_1 + max_2 + 1);
}
