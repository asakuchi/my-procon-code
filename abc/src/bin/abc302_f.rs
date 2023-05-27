use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [[Usize1]; n],
    }

    // 仮想頂点を追加する 0~m-1

    let mut list = vec![Vec::new(); n + m];

    for i in 0..n {
        for &u in s[i].iter() {
            list[i].push(n + u);
            list[n + u].push(i);
        }
    }

    let mut result: Option<usize> = None;

    let mut visited = vec![false; n + m];

    let mut queue = VecDeque::new();
    queue.push_back((n, 0));
    visited[n] = true;

    'seach: while let Some((current, step)) = queue.pop_front() {
        for &next in list[current].iter() {
            // OK
            if next == n + m - 1 {
                if let Some(min_value) = result {
                    result = Some(min_value.min(step / 2));
                } else {
                    result = Some(step / 2);
                }

                break 'seach;
            }

            if visited[next] {
                continue;
            }

            visited[next] = true;

            queue.push_back((next, step + 1));
        }
    }

    if let Some(value) = result {
        println!("{}", value);
    } else {
        println!("-1");
    }
}
