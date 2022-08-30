use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut result = -1;

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                continue;
            }

            let mut visited = vec![vec![false; w]; h];

            let score = rec(h, w, &c, (i, j), (i, j), 0, &mut visited);

            result = result.max(score);
        }
    }

    println!("{}", result);
}

fn rec(
    h: usize,
    w: usize,
    c: &Vec<Vec<char>>,
    start: (usize, usize),
    current: (usize, usize),
    step: isize,
    visited: &mut Vec<Vec<bool>>,
) -> isize {
    if current == start && step != 0 {
        if step < 3 {
            return -1;
        } else {
            return step;
        }
    }

    let mut result = -1;

    let patterns = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    for &pattern in &patterns {
        let next = (
            current.0 as isize + pattern.0,
            current.1 as isize + pattern.1,
        );

        if next.0 < 0 || next.1 < 0 || next.0 >= h as isize || next.1 >= w as isize {
            continue;
        }

        let next = (next.0 as usize, next.1 as usize);

        if visited[next.0][next.1] {
            continue;
        }

        if c[next.0][next.1] == '#' {
            continue;
        }

        visited[next.0][next.1] = true;

        let score = rec(h, w, c, start, next, step + 1, visited);

        result = result.max(score);

        visited[next.0][next.1] = false;
    }

    result
}
