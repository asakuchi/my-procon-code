use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![vec![None; d]; k + 1]; n + 1];

    let result = rec(n, d, &a, k, 0, 0, &mut dp);

    if let Some(value) = result {
        println!("{}", value);
    } else {
        println!("-1");
    }
}

fn rec(
    n: usize,
    d: usize,
    a: &Vec<usize>,
    k: usize,
    index: usize,
    amari: usize,
    dp: &mut Vec<Vec<Vec<Option<Option<usize>>>>>,
) -> Option<usize> {
    if index == n {
        if k == 0 && amari == 0 {
            return Some(0);
        } else {
            return None;
        }
    }

    if let Some(value) = dp[index][k][amari] {
        return value;
    }

    // 使う
    let score_1 = if k > 0 {
        if let Some(value) = rec(n, d, a, k - 1, index + 1, (amari + a[index]) % d, dp) {
            Some(value + a[index])
        } else {
            None
        }
    } else {
        None
    };

    // 使わない
    let score_2 = rec(n, d, a, k, index + 1, amari, dp);

    let score = score_1.max(score_2);

    dp[index][k][amari] = Some(score);

    score
}
