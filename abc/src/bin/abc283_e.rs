use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut dp = vec![vec![vec![None; 2]; 2]; h + 1];

    let result_1 = rec(h, w, &a, 0, false, false, &mut dp);
    let result_2 = rec(h, w, &a, 0, false, true, &mut dp);

    if let Some(score_1) = result_1 {
        if let Some(score_2) = result_2 {
            println!("{}", score_1.min(score_2 + 1));
        } else {
            println!("{}", score_1);
        }
    } else {
        if let Some(score_2) = result_2 {
            println!("{}", score_2 + 1);
        } else {
            println!("-1");
        }
    }
}

fn rec(
    h: usize,
    w: usize,
    a: &Vec<Vec<usize>>,
    index: usize,
    prev_exec: bool,
    current_exec: bool,
    dp: &mut Vec<Vec<Vec<Option<Option<usize>>>>>,
) -> Option<usize> {
    if index == h {
        return Some(0);
    }

    if let Some(value) = dp[index][prev_exec as usize][current_exec as usize] {
        return value;
    }

    let mut score: Option<usize> = None;

    // 操作をする
    if judge(h, w, a, index, prev_exec, current_exec, true) {
        if let Some(score_1) = rec(h, w, a, index + 1, current_exec, true, dp) {
            score = Some(score_1 + 1);
        }
    }

    // 操作をしない
    if judge(h, w, a, index, prev_exec, current_exec, false) {
        if let Some(score_2) = rec(h, w, a, index + 1, current_exec, false, dp) {
            if let Some(score_1) = score {
                score = Some(score_1.min(score_2));
            } else {
                score = Some(score_2);
            }
        }
    }

    dp[index][prev_exec as usize][current_exec as usize] = Some(score);

    score
}

fn judge(
    h: usize,
    w: usize,
    a: &Vec<Vec<usize>>,
    index: usize,
    prev_exec: bool,
    current_exec: bool,
    next_exec: bool,
) -> bool {
    let i = index;

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for j in 0..w {
        let mut next_count = 0;

        for &direction in &directions {
            let (x, y) = (i as isize + direction.0, j as isize + direction.1);

            if x < 0 || x >= h as isize || y < 0 || y >= w as isize {
                continue;
            }

            let (x, y) = (x as usize, y as usize);

            let source = if current_exec { 1 - a[i][j] } else { a[i][j] };

            let target = if x == index {
                if current_exec {
                    1 - a[x][y]
                } else {
                    a[x][y]
                }
            } else if x < index {
                if prev_exec {
                    1 - a[x][y]
                } else {
                    a[x][y]
                }
            } else {
                if next_exec {
                    1 - a[x][y]
                } else {
                    a[x][y]
                }
            };

            if source == target {
                next_count += 1;
            }
        }

        if next_count == 0 {
            return false;
        }
    }

    true
}
