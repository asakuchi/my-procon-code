use proconio::input;

use ac_library_rs::ModInt998244353 as mint;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut dp = vec![vec![mint::from(0); 2]; n + 1];

    dp[0][1] = mint::from(m);

    for i in 1..n {
        let pre_0 = dp[i - 1][0].clone();
        let pre_1 = dp[i - 1][1].clone();

        dp[i][1] += pre_0;

        dp[i][0] += pre_0 * (m - 2);
        dp[i][0] += pre_1 * (m - 1);
    }

    println!("{}", dp[n - 1][0]);
}
