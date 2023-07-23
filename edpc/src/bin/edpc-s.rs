use proconio::{input, marker::Chars};

use ac_library_rs::ModInt1000000007 as mint;

fn main() {
    input! {
        k: Chars,
        d: usize,
    }

    let n = k.len();

    let mut dp = vec![vec![vec![mint::from(0); d]; 2]; n + 1];

    dp[0][0][0] = mint::from(1);

    for i in 0..n {
        let num: usize = k[i].to_string().parse().unwrap();

        for smaller in vec![false, true] {
            let mut x = 0;

            while x <= if smaller { 9 } else { num } {
                for y in 0..d {
                    let prev = dp[i][smaller as usize][y].clone();

                    dp[i + 1][(smaller || x < num) as usize][(x + y) % d] += prev;
                }

                x += 1;
            }
        }
    }

    // 「0」が含まれるので除外
    println!("{}", dp[n][0][0] + dp[n][1][0] - mint::from(1));
}
