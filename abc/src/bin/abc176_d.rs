use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

const INF: usize = 1_000_000_000_000;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: (Usize1, Usize1),
        d: (Usize1, Usize1),
        s: [Chars; h],
    }

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut steps = vec![vec![INF; w]; h];

    let mut queue = VecDeque::new();
    queue.push_back(c);
    steps[c.0][c.1] = 0;

    while let Some(current) = queue.pop_front() {
        // 終了条件
        if current == d {
            println!("{}", steps[current.0][current.1]);
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
            if s[next.0][next.1] == '.' {
                if steps[next.0][next.1] > steps[current.0][current.1] {
                    steps[next.0][next.1] = steps[current.0][current.1];
                    queue.push_front(next);
                }
            }
        }

        // warp
        for i in -2..=2 {
            for j in -2..=2 {
                let next = (current.0 as isize + i, current.1 as isize + j);

                if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
                    continue;
                }

                let next = (next.0 as usize, next.1 as usize);

                // 次へ
                if s[next.0][next.1] == '.' {
                    if steps[next.0][next.1] > steps[current.0][current.1] + 1 {
                        steps[next.0][next.1] = steps[current.0][current.1] + 1;
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    println!("-1");
}
