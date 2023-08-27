use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

fn bfs_grid() {
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h],
    }

    let start = (0, 0);
    let goal = (h - 1, w - 1);

    // ----------------------------------

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut visited = vec![vec![false; w]; h];

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited[start.0][start.1] = true;

    // let mut steps = vec![vec![usize::MAX; w]; h];
    // visited[start.0][start.1] = 0;

    while let Some((current, step)) = queue.pop_front() {
        // 終了条件
        if current == goal {
            println!("{}", step);
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

            if a[next.0][next.1] == '#' {
                continue;
            }

            if visited[next.0][next.1] {
                continue;
            }

            // 次へ
            visited[next.0][next.1] = true;
            queue.push_back((next, step + 1));
        }
    }

    println!("-1");
}

fn bfs_graph() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
    }

    // ----------------------------------

    let start = 0;

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    // ----------------------------------

    let mut visited = vec![false; n];

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited[start] = true;

    // let mut steps = vec![usize::MAX; n];
    // steps[start] = 0;

    while let Some((current, step)) = queue.pop_front() {
        for &next in list[current].iter() {
            if visited[next] {
                continue;
            }

            // 次へ
            visited[next] = true;
            queue.push_back((next, step + 1));
        }
    }
}

fn main() {
    bfs_graph();
    bfs_grid();

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
