use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        u_v_a: [(Usize1, Usize1, usize); m],
        s: [Usize1; k],
    }

    let mut list = vec![Vec::new(); n];
    let mut has_switch = vec![false; n];

    for &(u, v, a) in &u_v_a {
        list[u].push((v, a == 1));
        list[v].push((u, a == 1));
    }

    for v in s {
        has_switch[v] = true;
    }

    let mut steps = vec![vec![0; 2]; n];

    let mut visited = vec![vec![false; 2]; n];

    let mut queue = VecDeque::new();
    queue.push_back((0, false));
    visited[0][false as usize] = true;

    while let Some((current, switch)) = queue.pop_front() {
        // 終了条件
        if current == n - 1 {
            println!("{}", steps[current][switch as usize]);
            return;
        }

        if has_switch[current] {
            if !visited[current][(!switch) as usize] {
                visited[current][(!switch) as usize] = true;
                steps[current][(!switch) as usize] = steps[current][switch as usize];
                queue.push_back((current, (!switch)));
            }
        }

        for &(next, initial_allow) in list[current].iter() {
            if switch {
                if initial_allow {
                    continue;
                }
            } else {
                if !initial_allow {
                    continue;
                }
            }

            if visited[next][switch as usize] {
                continue;
            }

            visited[next][switch as usize] = true;
            steps[next][switch as usize] = steps[current][switch as usize] + 1;
            queue.push_back((next, switch));
        }
    }

    println!("-1");
}
