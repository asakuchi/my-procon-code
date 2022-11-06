use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut start = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                start = (i, j);
                break;
            }
        }
    }

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut start_candi_list = Vec::new();

    for i in 0..4 {
        let pattern = patterns[i];

        let next = (start.0 as isize + pattern.0, start.1 as isize + pattern.1);

        if next.0 < 0 || next.0 >= h as isize || next.1 < 0 || next.1 >= w as isize {
            continue;
        }

        let next = (next.0 as usize, next.1 as usize);

        if c[next.0][next.1] == '.' {
            start_candi_list.push(next);
        }
    }

    if start_candi_list.len() <= 1 {
        println!("No");
        return;
    }

    for &start_candi in &start_candi_list {
        let mut steps = vec![vec![0; w]; h];

        let mut visited = vec![vec![false; w]; h];

        let mut queue = VecDeque::new();
        queue.push_back(start_candi);
        visited[start_candi.0][start_candi.1] = true;

        while let Some(current) = queue.pop_front() {
            // 終了条件
            if steps[current.0][current.1] != 0 {
                if start_candi_list.contains(&current) {
                    println!("Yes");
                    return;
                }
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

                if c[next.0][next.1] == '#' {
                    continue;
                }

                if visited[next.0][next.1] {
                    continue;
                }

                // 次へ
                if next != start {
                    visited[next.0][next.1] = true;
                    steps[next.0][next.1] = steps[current.0][current.1] + 1;
                    queue.push_back(next);
                }
            }
        }
    }

    println!("No");
}
