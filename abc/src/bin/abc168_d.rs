use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    let mut result = vec![0; n];

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
            result[next] = current;
            queue.push_back(next);
        }
    }

    println!("Yes");

    for i in 1..n {
        println!("{}", result[i] + 1);
    }
}
