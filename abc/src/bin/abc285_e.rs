use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = vec![0; n];

    for i in 1..n {
        b[i] = b[i - 1] + a[(i - 1) / 2];
    }

    let mut dp = vec![vec![None; n + 1]; n + 1];

    let result = rec(n, &b, 1, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    b: &Vec<usize>,
    index: usize,
    day_streak: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if index == n {
        return b[day_streak];
    }

    if let Some(value) = dp[index][day_streak] {
        return value;
    }

    // 平日にする
    let score_1 = rec(n, b, index + 1, day_streak + 1, dp);
    // 休日にする
    let score_2 = rec(n, b, index + 1, 0, dp) + b[day_streak];

    let result = score_1.max(score_2);

    dp[index][day_streak] = Some(result);

    result
}
