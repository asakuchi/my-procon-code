use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let snuke = vec!['s', 'n', 'u', 'k', 'e'];

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));
    visited.insert((0, 0));

    while let Some((current, snuke_index)) = queue.pop_front() {
        // 終了条件
        if current == (h - 1, w - 1) {
            println!("Yes");
            return;
        }

        for pattern in &patterns {
            let next = (
                current.0 as isize + pattern.0,
                current.1 as isize + pattern.1,
            );

            if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            let mut next_index = snuke_index + 1;

            if next_index > 4 {
                next_index = 0;
            }

            if s[next.0][next.1] != snuke[next_index] {
                continue;
            }

            if visited.contains(&next) {
                continue;
            }

            // 次へ
            visited.insert(next);
            queue.push_back((next, next_index));
        }
    }

    println!("No");
}
