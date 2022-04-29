use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const MODULO: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        d: usize,
        n: Chars,
    }

    let mut dp = vec![vec![vec![0; d + 1]; 2]; n.len() + 1];

    dp[0][0][0] = 1;

    let size = n.len();

    for i in 0..size {
        let num: usize = n[i].to_string().parse().unwrap();

        for smaller in vec![false, true] {
            for mod_d in 0..d {
                let mut x = 0;

                while x <= if smaller { 9 } else { num } {
                    dp[i + 1][(smaller || x < num) as usize][((mod_d + x) % d) as usize] +=
                        dp[i][smaller as usize][mod_d as usize];

                    dp[i + 1][(smaller || x < num) as usize][((mod_d + x) % d) as usize] %= MODULO;

                    x += 1;
                }
            }
        }
    }

    // N 以下の正整数を求めるので0の分を引く
    println!("{}", dp[size][0][0] + dp[size][1][0] - 1);
}
