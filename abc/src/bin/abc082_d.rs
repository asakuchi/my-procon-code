use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        goal: (isize,isize),
    }

    s.push('T'); // 番兵

    let mut start = (0, 0);

    let mut mode = 0; // 初期座標モード

    let mut volume = 0;

    let mut x_list = Vec::with_capacity(s.len());
    let mut y_list = Vec::with_capacity(s.len());

    for &c in &s {
        if c == 'F' {
            volume += 1;
        } else {
            if mode == 0 {
                start = (volume, 0);
                mode = 2; // y 軸モード
            } else if mode == 1 {
                x_list.push(volume);
                mode = 2; // y 軸モード
            } else {
                y_list.push(volume);
                mode = 1; // x 軸モード
            }

            volume = 0;
        }
    }

    let mut dp = vec![vec![None; 16_050]; 8_000];

    let x_result = rec(x_list.len(), goal.0, &x_list, 0, start.0, &mut dp);

    let mut dp = vec![vec![None; 16_050]; 8_000];

    let y_result = rec(y_list.len(), goal.1, &y_list, 0, start.1, &mut dp);

    if x_result && y_result {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn rec(
    n: usize,
    goal: isize,
    list: &Vec<isize>,
    index: usize,
    current: isize,
    dp: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if index == n {
        return current == goal;
    }

    if let Some(value) = dp[index][(current + 8_000) as usize] {
        return value;
    }

    let diff = list[index];

    for pattern in vec![-diff, diff] {
        let next = current + pattern;

        if rec(n, goal, list, index + 1, next, dp) {
            return true;
        }
    }

    dp[index][(current + 8_000) as usize] = Some(false);

    false
}
