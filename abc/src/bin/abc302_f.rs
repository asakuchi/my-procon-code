use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [[Usize1]; n],
    }

    // 仮想頂点を追加する 0~m-1

    let mut start_list = Vec::new();

    let mut list = vec![Vec::new(); n + m];

    for i in 0..n {
        for &u in s[i].iter() {
            if u == 0 {
                start_list.push(i);
            }

            list[i].push((n + u, 0));
            list[n + u].push((i, 1));
        }
    }

    let mut result: Option<usize> = None;

    for start in start_list {
        let mut visited = vec![false; n + m];

        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        visited[start] = true;

        'seach: while let Some((current, step)) = queue.pop_front() {
            for &(next, cost) in list[current].iter() {
                // OK
                if next == n + m - 1 {
                    if let Some(min_value) = result {
                        result = Some(min_value.min(step));
                    } else {
                        result = Some(step);
                    }

                    break 'seach;
                }

                if visited[next] {
                    continue;
                }

                visited[next] = true;

                if cost == 0 {
                    queue.push_front((next, step));
                } else {
                    queue.push_back((next, step + 1));
                }
            }
        }
    }

    if let Some(value) = result {
        println!("{}", value);
    } else {
        println!("-1");
    }
}
