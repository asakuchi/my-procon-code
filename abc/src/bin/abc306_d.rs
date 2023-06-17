use proconio::input;

const INF: isize = 1_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        x_y: [(usize, isize); n],
    }

    let mut dp = vec![vec![None; 2]; n];

    let result = rec(n, &x_y, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    x_y: &Vec<(usize, isize)>,
    index: usize,
    status: usize,
    dp: &mut Vec<Vec<Option<isize>>>,
) -> isize {
    if index == n {
        return 0;
    }

    if let Some(value) = dp[index][status] {
        return value;
    }

    let (x, y) = x_y[index];

    // 食べる
    let score_1 = if status == 0 {
        // 元気
        rec(n, x_y, index + 1, if x == 0 { 0 } else { 1 }, dp) + y
    } else {
        // お腹壊してる
        if x == 0 {
            // 解毒
            rec(n, x_y, index + 1, 0, dp) + y
        } else {
            // 死ぬ
            -INF
        }
    };

    // 食べない

    let score_2 = rec(n, x_y, index + 1, status, dp);

    let score = score_1.max(score_2);

    dp[index][status] = Some(score);

    score
}
