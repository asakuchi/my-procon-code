use proconio::input;

use ac_library_rs::ModInt998244353 as mint;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
    }

    let mut dp = vec![vec![vec![None; 2]; n + 1]; n + 1];

    let result = rec(n, p, q, a, b, true, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    p: usize,
    q: usize,
    taka: usize,
    aoki: usize,
    taka_turn: bool,
    dp: &mut Vec<Vec<Vec<Option<mint>>>>,
) -> mint {
    if taka == n && aoki < n {
        return mint::from(1);
    }

    if aoki == n {
        return mint::from(0);
    }

    if let Some(value) = dp[taka][aoki][taka_turn as usize] {
        return value;
    }

    let mut result = mint::from(0);

    if taka_turn {
        for i in 1..=p {
            result += rec(n, p, q, (taka + i).min(n), aoki, false, dp) / mint::from(p);
        }
    } else {
        for i in 1..=q {
            result += rec(n, p, q, taka, (aoki + i).min(n), true, dp) / mint::from(q);
        }
    }

    dp[taka][aoki][taka_turn as usize] = Some(result);

    result
}
