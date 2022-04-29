//!
//! 桁DP
//!
//! 0以上N以下の整数で、いずれかの桁に3を含むものの個数を求めよ
//!

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let mut dp = vec![vec![vec![0; 2]; 2]; n.len() + 1];

    dp[0][0][0] = 1;

    let size = n.len();

    for i in 0..size {
        let num: usize = n[i].to_string().parse().unwrap();

        for smaller in vec![false, true] {
            for has_3 in vec![false, true] {
                let mut x = 0;

                while x <= if smaller { 9 } else { num } {
                    dp[i + 1][(smaller || x < num) as usize][(has_3 || x == 3) as usize] +=
                        dp[i][smaller as usize][has_3 as usize];

                    x += 1;
                }
            }
        }
    }

    println!("{}", dp[size][0][1] + dp[size][1][1]);
}
