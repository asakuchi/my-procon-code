use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut counter = vec![0; 4];

    for i in 0..n {
        counter[a[i]] += 1;
    }

    // dp[i][j][k]
    // 寿司が1個の皿がi個、寿司が2個の皿がj個、寿司が3個の皿がk個、の期待値
    let mut dp = vec![vec![vec![Option::None; n + 3]; n + 3]; n + 3];

    let result = rec(n, counter[1], counter[2], counter[3], &mut dp);

    println!("{}", result);
}

fn rec(n: usize, one: usize, two: usize, three: usize, dp: &mut Vec<Vec<Vec<Option<f64>>>>) -> f64 {
    if let Some(value) = dp[one][two][three] {
        return value;
    }

    if one == 0 && two == 0 && three == 0 {
        return 0.;
    }

    let mut result = 0.;

    result += n as f64 / (one + two + three) as f64;

    if one >= 1 {
        result += rec(n, one - 1, two, three, dp) * one as f64 / (one + two + three) as f64;
    }

    if two >= 1 {
        result += rec(n, one + 1, two - 1, three, dp) * two as f64 / (one + two + three) as f64;
    }

    if three >= 1 {
        result += rec(n, one, two + 1, three - 1, dp) * three as f64 / (one + two + three) as f64;
    }

    dp[one][two][three] = Some(result);

    result
}
