use proconio::input;

fn main() {
    input! {
        n: usize,
        x_y_z: [(usize, usize, usize); n],
    }

    let mut total_giseki = 0;

    for &(_, _, z) in &x_y_z {
        total_giseki += z;
    }

    let mut dp = vec![vec![1_000_000_000_000; total_giseki + 1]; n + 1];

    dp[0][0] = 0;

    for i in 1..=n {
        let (x, y, z) = x_y_z[i - 1];

        // 何もせず高橋勝利
        if x > y {
            for j in 0..=total_giseki {
                if j >= z {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - z]);
                }
            }
        } else {
            // 鞍替え
            let person = (x + y + 1) / 2 - x;

            for j in 0..=total_giseki {
                if j >= z {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - z] + person);
                }
            }
        }

        // 何もしない
        for j in 0..=total_giseki {
            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
        }
    }

    let mut result = usize::MAX;

    for giseki in (total_giseki + 1) / 2..=total_giseki {
        result = result.min(dp[n][giseki]);
    }

    println!("{}", result);
}
