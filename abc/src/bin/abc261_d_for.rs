use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        c_y: [(usize, usize); m],
    }

    let mut map = HashMap::new();

    for &(c, y) in &c_y {
        map.entry(c).or_insert(y);
    }

    // dp[i][count]
    let mut dp = vec![vec![0; 5_001]; 5_001];

    dp[0][0] = 0;

    for i in 1..=n {
        let mut pre_max = dp[i - 1][0];

        for counter in 1..=i {
            let bonus = map.get(&counter).unwrap_or(&0);

            // 表を出す
            dp[i][counter] = dp[i - 1][counter - 1] + x[i - 1] + bonus;

            pre_max = pre_max.max(dp[i - 1][counter - 1]);
        }

        // 裏を出す
        dp[i][0] = pre_max;
    }

    let result = dp[n][..=n].iter().max().unwrap();

    println!("{}", result);
}
