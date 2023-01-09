use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let mut dp = vec![vec![-1; 1 << 21]; n + 1];

    let result = rec(n, &a, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    a: &Vec<Vec<usize>>,
    man: usize,
    visited: usize,
    dp: &mut Vec<Vec<isize>>,
) -> usize {
    if man == n {
        return 1;
    }

    if dp[man][visited] != -1 {
        return dp[man][visited] as usize;
    }

    let mut result = 0;

    for woman in 0..n {
        if visited & 1 << woman > 0 {
            continue;
        }

        if a[man][woman] == 0 {
            continue;
        }

        let score = rec(n, a, man + 1, visited | 1 << woman, dp) % MOD;

        result += score;
        result %= MOD;
    }

    dp[man][visited] = result as isize;

    result
}
