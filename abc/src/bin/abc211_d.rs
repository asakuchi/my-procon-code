use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

const MOD: usize = 1_000_000_007;

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

    let mut steps = vec![1_000_000_000; n];
    let mut count = vec![0; n];

    steps[0] = 0;
    count[0] = 1;

    let mut visited = vec![false; n];

    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;

    while let Some(current) = queue.pop_front() {
        // 終了条件
        if current == n - 1 {
            continue;
        }

        for &next in list[current].iter() {
            if visited[next] {
                if steps[next] == steps[current] + 1 {
                    count[next] += count[current];
                    count[next] %= MOD;
                }

                continue;
            }

            // 次へ
            visited[next] = true;
            steps[next] = steps[current] + 1;
            count[next] = count[current];
            count[next] %= MOD;
            queue.push_back(next);
        }
    }

    println!("{}", count[n - 1])
}
