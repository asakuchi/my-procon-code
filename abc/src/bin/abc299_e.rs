use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
        k: usize,
        p_d: [(Usize1, usize); k],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    // 全て黒にする
    let mut result = vec![1; n];

    // 白に色付け
    for &(p, d) in &p_d {
        let mut visited = vec![false; n];

        let mut queue = VecDeque::new();
        queue.push_back((p, 0));
        visited[p] = true;

        while let Some((current, length)) = queue.pop_front() {
            if length < d {
                // d以内は白でないとダメ
                result[current] = 0;
            } else {
                break;
            }

            for &next in list[current].iter() {
                if visited[next] {
                    continue;
                }

                // 次へ
                visited[next] = true;
                queue.push_back((next, length + 1));
            }
        }
    }

    // 条件を満たすかチェック
    for &(p, d) in &p_d {
        let mut visited = vec![false; n];

        let mut queue = VecDeque::new();
        queue.push_back((p, 0));
        visited[p] = true;

        let mut ok = false;

        while let Some((current, length)) = queue.pop_front() {
            if length == d {
                if result[current] == 1 {
                    ok = true;
                    break;
                }
            }

            for &next in list[current].iter() {
                if visited[next] {
                    continue;
                }

                // 次へ
                visited[next] = true;
                queue.push_back((next, length + 1));
            }
        }

        if !ok {
            println!("No");
            return;
        }
    }

    println!("Yes");
    println!("{}", result.iter().format(""));
}
