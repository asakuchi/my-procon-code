use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const INF: usize = 1_000_000_000_000;

fn main() {
    input! {
        h: usize,
        w: usize,
        table: [Chars; h],
    }

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut steps = vec![vec![INF; w]; h];

    let mut s = (1_000, 1_000);

    'search_start: for i in 0..h {
        for j in 0..w {
            if table[i][j] == 's' {
                s = (i, j);
                break 'search_start;
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(s);
    steps[s.0][s.1] = 0;

    while let Some(current) = queue.pop_front() {
        // 終了条件
        if table[current.0][current.1] == 'g' {
            if steps[current.0][current.1] <= 2 {
                println!("YES");
            } else {
                println!("NO");
            }

            // println!("{}", steps[current.0][current.1]);
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

            // 次へ
            if table[next.0][next.1] == '#' {
                if steps[next.0][next.1] > steps[current.0][current.1] + 1 {
                    steps[next.0][next.1] = steps[current.0][current.1] + 1;
                    queue.push_back(next);
                }
            } else {
                if steps[next.0][next.1] > steps[current.0][current.1] {
                    steps[next.0][next.1] = steps[current.0][current.1];
                    queue.push_front(next);
                }
            }
        }
    }

    panic!("not found");
}
