use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

const INF: usize = 1_000_000_000_000;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: (Usize1, Usize1),
        t: (Usize1, Usize1),
        table: [Chars; h],
    }

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut steps = vec![vec![vec![INF; 4]; w]; h];

    let mut queue = VecDeque::new();

    for i in 0..4 {
        queue.push_back((s, i));
        steps[s.0][s.1][i] = 0;
    }

    while let Some((current, direction)) = queue.pop_front() {
        // 回転
        for (i, &_pattern) in patterns.iter().enumerate() {
            if steps[current.0][current.1][i] > steps[current.0][current.1][direction] + 1 {
                steps[current.0][current.1][i] = steps[current.0][current.1][direction] + 1;
                queue.push_back((current, i));
            }
        }

        // 直進
        let next = (
            current.0 as isize + patterns[direction].0,
            current.1 as isize + patterns[direction].1,
        );

        if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
            continue;
        }

        let next = (next.0 as usize, next.1 as usize);

        if table[next.0][next.1] == '#' {
            continue;
        }

        if steps[next.0][next.1][direction] > steps[current.0][current.1][direction] {
            steps[next.0][next.1][direction] = steps[current.0][current.1][direction];
            queue.push_front((next, direction));
        }
    }

    let mut result = steps[t.0][t.1][0];

    for k in 0..4 {
        result = result.min(steps[t.0][t.1][k]);
    }

    println!("{}", result);
}
