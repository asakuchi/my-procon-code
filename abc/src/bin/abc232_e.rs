use proconio::input;

use ac_library_rs::ModInt998244353 as mint;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        p_1: (usize, usize),
        p_2: (usize, usize),
    }

    // dp[i][x][y]
    // i回まで操作したとき、x_1と一致している/していない、y_1と一致している/していない数
    let mut dp = vec![vec![vec![mint::from(0); 2]; 2]; k + 1];

    dp[0][1][1] = mint::from(1);

    for i in 1..=k {
        let prev_0_0 = dp[i - 1][0][0];
        let prev_1_0 = dp[i - 1][1][0];
        let prev_0_1 = dp[i - 1][0][1];
        let prev_1_1 = dp[i - 1][1][1];

        // 両方一致する
        dp[i][1][1] += prev_0_1;
        dp[i][1][1] += prev_1_0;

        // x だけ一致
        dp[i][1][0] += prev_1_1 * (h - 1);
        dp[i][1][0] += prev_1_0 * (h - 2);
        dp[i][1][0] += prev_0_0;

        // y だけ一致
        dp[i][0][1] += prev_1_1 * (w - 1);
        dp[i][0][1] += prev_0_1 * (w - 2);
        dp[i][0][1] += prev_0_0;

        // 両方一致しない
        dp[i][0][0] += prev_1_0 * (w - 1);
        dp[i][0][0] += prev_0_1 * (h - 1);
        dp[i][0][0] += prev_0_0 * (h - 2);
        dp[i][0][0] += prev_0_0 * (w - 2);
    }

    if p_1 == p_2 {
        println!("{}", dp[k][1][1]);
    } else if p_1.0 == p_2.0 {
        println!("{}", dp[k][0][1] / (w - 1));
    } else if p_1.1 == p_2.1 {
        println!("{}", dp[k][1][0] / (h - 1));
    } else {
        println!("{}", dp[k][0][0] / (h - 1) / (w - 1));
    }
}
