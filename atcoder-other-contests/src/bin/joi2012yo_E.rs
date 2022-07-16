use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        w: usize,
        h: usize,
        matrix_org: [[usize; w]; h],
    }

    let mut matrix = vec![vec![0; w + 2]; h + 2];

    // 外側を追加する
    for i in 0..h {
        for j in 0..w {
            matrix[i + 1][j + 1] = matrix_org[i][j];
        }
    }

    let w = w + 2;
    let h = h + 2;

    let patterns_even = vec![
        (-1, -1), // 北西
        (-1, 0),  // 北東
        (0, 1),   // 東
        (1, 0),   // 南東
        (1, -1),  // 南西
        (0, -1),  // 西
    ];

    let patterns_odd = vec![
        (-1, 0), // 北西
        (-1, 1), // 北東
        (0, 1),  // 東
        (1, 1),  // 南東
        (1, 0),  // 南西
        (0, -1), // 西
    ];

    let mut visited = vec![vec![false; w]; h];

    let mut result = 0;

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((0, 0));

    while let Some(target) = queue.pop_front() {
        if visited[target.0][target.1] {
            continue;
        }

        visited[target.0][target.1] = true;

        let patterns = match target.0 % 2 {
            0 => patterns_even.iter(),
            _ => patterns_odd.iter(),
        };

        for pattern in patterns {
            let next = (target.0 as isize + pattern.0, target.1 as isize + pattern.1);

            // 範囲外
            if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            if visited[next.0][next.1] {
                continue;
            }

            // 次へ
            if matrix[next.0][next.1] == 1 {
                result += 1;
            } else {
                queue.push_back(next);
            }
        }
    }

    println!("{}", result);
}
