use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut max_step: usize = 0;

    for i in 0..h {
        for j in 0..w {
            let start = (i, j);

            if s[start.0][start.1] == '#' {
                continue;
            }

            // let mut max_step_point;

            let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

            let mut steps = vec![vec![0; w]; h];

            let mut visited = vec![vec![false; w]; h];

            let mut queue = VecDeque::new();

            queue.push_back(start);
            visited[start.0][start.1] = true;

            while let Some(current) = queue.pop_front() {
                for pattern in &patterns {
                    let next = (
                        current.0 as isize + pattern.0,
                        current.1 as isize + pattern.1,
                    );

                    if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
                        continue;
                    }

                    let next = (next.0 as usize, next.1 as usize);

                    if s[next.0][next.1] == '#' {
                        continue;
                    }

                    if visited[next.0][next.1] {
                        continue;
                    }

                    // 次へ
                    visited[next.0][next.1] = true;
                    steps[next.0][next.1] = steps[current.0][current.1] + 1;
                    queue.push_back(next);

                    // 最大
                    if steps[next.0][next.1] > max_step {
                        max_step = steps[next.0][next.1];

                        // max_step_point = next;
                        // println!("start:{:?} {} {:?}", start, max_step, max_step_point);
                    }
                }
            }
        }
    }

    println!("{}", max_step);
}
