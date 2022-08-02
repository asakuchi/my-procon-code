//!
//! 区間DP
//!

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if n == 1 {
        println!("{}", a[0]);
        return;
    }

    let mut result = 0;
    let mut dp = vec![vec![vec![-1; n]; n]; 2];

    for i in 0..n {
        let score = rec(n, &a, i + 1, i, 1, &mut dp) + a[i];

        result = result.max(score);
    }

    println!("{}", result);
}

fn rec(
    n: usize,
    a: &Vec<usize>,
    left: usize,
    right: usize,
    player: usize,
    dp: &mut Vec<Vec<Vec<isize>>>,
) -> usize {
    let left = (left + n) % n;
    let right = (right + n) % n;

    let left_increment = (left + n + 1) % n;
    let right_decrement = (right + n - 1) % n;

    if left_increment == right {
        if player == 0 {
            return a[left];
        } else {
            return 0;
        }
    }

    if dp[player][left][right] != -1 {
        return dp[player][left][right] as usize;
    }

    let result = if player == 0 {
        // 左端をとる
        let score_1 = rec(n, a, left_increment, right, 1, dp) + a[left];
        // 右端をとる
        let score_2 = rec(n, a, left, right_decrement, 1, dp) + a[right_decrement];

        score_1.max(score_2)
    } else {
        if a[left] > a[right_decrement] {
            // 左端をとる
            rec(n, a, left_increment, right, 0, dp)
        } else {
            // 右端をとる
            rec(n, a, left, right_decrement, 0, dp)
        }
    };

    dp[player][left][right] = result as isize;

    result
}
