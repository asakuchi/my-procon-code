use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut visited = vec![vec![false; m]; n];
    let mut touch = vec![vec![false; m]; n];

    let mut queue = VecDeque::new();
    queue.push_back((1, 1));
    visited[1][1] = true;
    touch[1][1] = true;

    while let Some(current) = queue.pop_front() {
        for pattern in &patterns {
            let mut current_2 = current;

            loop {
                // ダメになるまで進む

                let next = (
                    current_2.0 as isize + pattern.0,
                    current_2.1 as isize + pattern.1,
                );

                // 領域外(ありえないけど)
                if 0 > next.0 || next.0 >= n as isize || 0 > next.1 || next.1 >= m as isize {
                    break;
                }

                let next = (next.0 as usize, next.1 as usize);

                // 岩
                if s[next.0][next.1] == '#' {
                    break;
                }

                current_2 = next;

                touch[current_2.0][current_2.1] = true;
            }

            if visited[current_2.0][current_2.1] {
                continue;
            }

            // 次へ
            visited[current_2.0][current_2.1] = true;
            queue.push_back(current_2);
        }
    }

    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            if touch[i][j] {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
