use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

fn main() {
    input! {
        r: usize,
        c: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        table: [Chars; r],
    }

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut steps = vec![vec![0; c]; r];

    let mut visited = vec![vec![false; c]; r];

    let mut queue = VecDeque::new();
    queue.push_back(s);
    visited[s.0][s.1] = true;

    while let Some(current) = queue.pop_front() {
        // 終了条件
        if current == g {
            println!("{}", steps[current.0][current.1]);
            return;
        }

        for pattern in &patterns {
            let next = (
                current.0 as isize + pattern.0,
                current.1 as isize + pattern.1,
            );

            if 0 > next.0 || next.0 >= r as isize || 0 > next.1 || next.1 >= c as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            if table[next.0][next.1] == '#' {
                continue;
            }

            if visited[next.0][next.1] {
                continue;
            }

            // 次へ
            visited[next.0][next.1] = true;
            steps[next.0][next.1] = steps[current.0][current.1] + 1;
            queue.push_back(next);
        }
    }

    panic!("goal not found");
}
