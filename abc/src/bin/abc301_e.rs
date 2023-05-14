use proconio::{input, marker::Chars};

use std::collections::VecDeque;

const INF: usize = 1_000_000_000_000_000;

fn main() {
    input! {
        h: usize,
        w: usize,
        t: usize,
        a: [Chars; h],
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut candy_positions = Vec::new();

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                start = (i, j);
            } else if a[i][j] == 'G' {
                goal = (i, j);
            } else if a[i][j] == 'o' {
                candy_positions.push((i, j));
            }
        }
    }

    let mut positions = Vec::new();

    positions.push(start);
    positions.push(goal);

    positions.append(&mut candy_positions.clone());

    let n = positions.len();

    let mut list = vec![vec![INF; n]; n];

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for i in 0..n {
        let mut visited = vec![vec![false; w]; h];

        let i_position = positions[i];

        let mut queue = VecDeque::new();
        queue.push_back((i_position, 0));
        visited[i_position.0][i_position.1] = true;

        while let Some((current, step)) = queue.pop_front() {
            if let Some(j) = positions.iter().position(|&x| x == current) {
                list[i][j] = step;
                list[j][i] = step;
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
    }

    // start: 0
    // goal: 1
    let mut dp = vec![vec![INF; n]; 1 << n];

    dp[0][0] = 0;

    for s in 0..1 << n {
        for v in 0..n {
            for u in 0..n {
                dp[s | 1 << u][u] = dp[s | 1 << u][u].min(dp[s][v] + list[v][u]);
            }
        }
    }

    let mut result: Option<usize> = None;

    // ゴールに到達したときに最短距離がt以下のものを探す
    for mask in 0..1 << n {
        if dp[mask][1] <= t {
            let mut candy = 0_usize;

            // candyは頂点2から
            for i in 2..n {
                if mask & 1 << i > 0 {
                    candy += 1;
                }
            }

            if let Some(max_value) = result {
                result = Some(max_value.max(candy));
            } else {
                result = Some(candy);
            }
        }
    }

    if let Some(value) = result {
        println!("{}", value);
    } else {
        println!("-1");
    }
}
