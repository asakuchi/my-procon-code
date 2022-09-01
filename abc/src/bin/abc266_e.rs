use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![None; n + 1];

    println!("{}", rec(n, &mut dp));
}

fn rec(n: usize, dp: &mut Vec<Option<f64>>) -> f64 {
    if n == 0 {
        return 0.;
    }

    if let Some(value) = dp[n] {
        return value;
    }

    let mut result = 0.;

    for i in 1..=6 {
        let select = i as f64;
        let unselect = rec(n - 1, dp);

        if select > unselect {
            result += select / 6.;
        } else {
            result += unselect / 6.;
        }
    }

    dp[n] = Some(result);

    result
}
