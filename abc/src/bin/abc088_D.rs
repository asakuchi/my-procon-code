use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        matrix: [Chars; h],
    }

    // let mut black_count = 0;
    let mut white_count = 0;

    for i in 0..h {
        for j in 0..w {
            if matrix[i][j] == '#' {
                // black_count += 1;
            } else {
                white_count += 1;
            }
        }
    }

    let start = (0, 0);
    let goal = (h - 1, w - 1);

    let patterns = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut length = vec![vec![-1; w]; h];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    length[0][0] = 1;

    let mut goal_length = -1;

    while let Some(target) = queue.pop_front() {
        // 終了条件
        if target.0 == goal.0 && target.1 == goal.1 {
            goal_length = length[target.0][target.1];
            break;
        }

        for pattern in patterns.iter() {
            let next = (target.0 as isize + pattern.0, target.1 as isize + pattern.1);

            // 範囲外
            if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            if matrix[next.0][next.1] == '#' || length[next.0][next.1] != -1 {
                continue;
            }

            // 次へ
            length[next.0][next.1] = length[target.0][target.1] + 1;
            queue.push_back(next);
        }
    }

    if goal_length != -1 {
        println!("{}", white_count - goal_length);
    } else {
        println!("-1");
    }
}
