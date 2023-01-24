use proconio::input;

// const INF: usize = 1_000_000_000_000;

fn main() {
    input! {
        d: usize,
        g: usize,
        p_c: [(usize, usize); d],
    }

    // dp[問題の種類][何問解いたか]
    let mut dp = vec![vec![None; 1_200]; d + 1];

    dp[0][0] = Some(0);

    for i in 1..=d {
        let (p, c) = p_c[i - 1];

        for count in 0..=1_000 {
            for j in 0..=p {
                if let Some(prev) = dp[i - 1][count] {
                    let updating_score = prev + 100 * i * j + if j == p { c } else { 0 };

                    if let Some(next) = dp[i][count + j] {
                        dp[i][count + j] = Some(next.max(updating_score));
                    } else {
                        dp[i][count + j] = Some(updating_score);
                    }
                }
            }
        }
    }

    for count in 1..=1_000 {
        if let Some(score) = dp[d][count] {
            if score >= g {
                println!("{}", count);
                return;
            }
        }
    }
}
