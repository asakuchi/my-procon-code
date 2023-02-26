use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n],
    }

    let mut dp = vec![vec![None; 2]; n + 1];

    let result = rec(n, &a_b, 0, false, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    a_b: &Vec<(usize, usize)>,
    index: usize,
    prev: bool,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if index == n {
        return 1;
    }

    if let Some(value) = dp[index][prev as usize] {
        return value;
    }

    let mut result = 0;

    let prev_card = if index == 0 {
        0 // 意味のない値
    } else if prev {
        a_b[index - 1].1
    } else {
        a_b[index - 1].0
    };

    // そのまま
    let score_1 = if index == 0 || a_b[index].0 != prev_card {
        rec(n, a_b, index + 1, false, dp) % MOD
    } else {
        0
    };

    result += score_1;
    result %= MOD;

    // 裏返す
    let score_2 = if index == 0 || a_b[index].1 != prev_card {
        rec(n, a_b, index + 1, true, dp) % MOD
    } else {
        0
    };

    result += score_2;
    result %= MOD;

    dp[index][prev as usize] = Some(result);

    result
}
