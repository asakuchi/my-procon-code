use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let punch_pattern = vec![
        (-1, 2),
        (0, 2),
        (1, 2),
        (-2, 1),
        (-1, 1),
        (0, 1),
        (1, 1),
        (2, 1),
        (-2, 0),
        (-1, 0),
        (1, 0),
        (2, 0),
        (-2, -1),
        (-1, -1),
        (0, -1),
        (1, -1),
        (2, -1),
        (-1, -2),
        (0, -2),
        (1, -2),
    ];

    let mut visited = vec![vec![false; w]; h];

    let mut queue = VecDeque::new();
    queue.push_back((0, (0, 0)));

    while let Some((punch_count, (i, j))) = queue.pop_front() {
        if visited[i][j] {
            continue;
        }

        visited[i][j] = true;

        // 終了条件
        if i == h - 1 && j == w - 1 {
            println!("{}", punch_count);
            return;
        }

        for pattern in &patterns {
            let next = (i as isize + pattern.0, j as isize + pattern.1);

            if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            // 次へ
            if s[next.0][next.1] == '.' {
                queue.push_front((punch_count, next));
            }
        }

        for pattern in &punch_pattern {
            let next = (i as isize + pattern.0, j as isize + pattern.1);

            if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            // 次へ
            queue.push_back((punch_count + 1, next));
        }
    }

    println!("-1");
}
