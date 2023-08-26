use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h],
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                start = (i, j);
            }

            if a[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                '>' => {
                    for l in 1.. {
                        let next = (i as isize, j as isize + l);

                        if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize
                        {
                            break;
                        }

                        let next = (next.0 as usize, next.1 as usize);

                        if a[next.0][next.1] != '#'
                            && a[next.0][next.1] != '>'
                            && a[next.0][next.1] != 'v'
                            && a[next.0][next.1] != '<'
                            && a[next.0][next.1] != '^'
                        {
                            a[next.0][next.1] = 'E'; // 視界
                        } else {
                            break;
                        }
                    }
                }
                'v' => {
                    for l in 1.. {
                        let next = (i as isize + l, j as isize);

                        if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize
                        {
                            break;
                        }

                        let next = (next.0 as usize, next.1 as usize);

                        if a[next.0][next.1] != '#'
                            && a[next.0][next.1] != '>'
                            && a[next.0][next.1] != 'v'
                            && a[next.0][next.1] != '<'
                            && a[next.0][next.1] != '^'
                        {
                            a[next.0][next.1] = 'E'; // 視界
                        } else {
                            break;
                        }
                    }
                }
                '<' => {
                    for l in 1.. {
                        let next = (i as isize, j as isize - l);

                        if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize
                        {
                            break;
                        }

                        let next = (next.0 as usize, next.1 as usize);

                        if a[next.0][next.1] != '#'
                            && a[next.0][next.1] != '>'
                            && a[next.0][next.1] != 'v'
                            && a[next.0][next.1] != '<'
                            && a[next.0][next.1] != '^'
                        {
                            a[next.0][next.1] = 'E'; // 視界
                        } else {
                            break;
                        }
                    }
                }
                '^' => {
                    for l in 1.. {
                        let next = (i as isize - l, j as isize);

                        if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize
                        {
                            break;
                        }

                        let next = (next.0 as usize, next.1 as usize);

                        if a[next.0][next.1] != '#'
                            && a[next.0][next.1] != '>'
                            && a[next.0][next.1] != 'v'
                            && a[next.0][next.1] != '<'
                            && a[next.0][next.1] != '^'
                        {
                            a[next.0][next.1] = 'E'; // 視界
                        } else {
                            break;
                        }
                    }
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut visited = vec![vec![false; w]; h];

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited[start.0][start.1] = true;

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

            if visited[next.0][next.1] {
                continue;
            }

            if a[next.0][next.1] == '.' || a[next.0][next.1] == 'G' {
                // 次へ
                visited[next.0][next.1] = true;
                queue.push_back((next, step + 1));
            }
        }
    }

    println!("-1");
}
