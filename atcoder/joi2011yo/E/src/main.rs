// use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut maze: [Chars; h],
    }

    let mut visited = vec![vec![false; w]; h];
    let mut lengths = vec![vec![0; w]; h];

    let patterns = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut start = (0, 0);

    'start: for i in 0..h {
        for j in 0..w {
            if maze[i][j] == 'S' {
                start = (i, j);
                break 'start;
            }
        }
    }

    let mut result = 0;

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);

    lengths[start.0][start.1] = 0;

    'cheese: for cheese in 1..=n {
        let mut goal = (0, 0);

        'goal: for i in 0..h {
            for j in 0..w {
                if maze[i][j] == cheese.to_string().parse().unwrap() {
                    goal = (i, j);
                    break 'goal;
                }
            }
        }

        while let Some(target) = queue.pop_front() {
            // println!("target:{:?} goal:{:?}", target, goal);

            // 終了条件
            if target.0 == goal.0 && target.1 == goal.1 {
                let tmp = lengths[target.0][target.1];

                reset(&mut lengths, &mut visited, h, w);

                lengths[target.0][target.1] = tmp;
                visited[target.0][target.1] = true;

                queue.clear();
                queue.push_back(target);

                result = lengths[target.0][target.1];

                continue 'cheese;
            }

            for pattern in patterns.iter() {
                let next = (target.0 as isize + pattern.0, target.1 as isize + pattern.1);

                if next.0 < 0 || next.0 >= h as isize || next.1 < 0 || next.1 >= w as isize {
                    continue;
                }

                let next = (next.0 as usize, next.1 as usize);

                // println!("next:{:?}", next);

                if maze[next.0][next.1] == 'X' || visited[next.0][next.1] {
                    continue;
                }

                lengths[next.0][next.1] = lengths[target.0][target.1] + 1;
                visited[next.0][next.1] = true;

                queue.push_back(next);
            }
        }
    }

    println!("{}", result);
}

fn reset(length: &mut Vec<Vec<usize>>, visited: &mut Vec<Vec<bool>>, h: usize, w: usize) {
    for i in 0..h {
        for j in 0..w {
            length[i][j] = 0;
        }
    }

    for i in 0..h {
        for j in 0..w {
            visited[i][j] = false;
        }
    }
}
