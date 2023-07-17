// 解説AC

use proconio::input;

use ac_library_rs::ModInt998244353 as mint;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // サイコロの出目の和の管理するものの最大値
    let m = 10;

    let m2 = 1 << (m + 1);

    // dp[i][S]
    // i番目のサイコロまで振った時、0から10のうち作れるものの集合がSになる確率
    let mut dp = vec![mint::from(0); m2];

    dp[1 << 0] = mint::from(1);

    for i in 0..n {
        let mut p = vec![mint::from(0); m2];

        std::mem::swap(&mut dp, &mut p);

        for s in 0..m2 {
            p[s] /= a[i];

            for x in 1..=a[i].min(m) {
                dp[(s | s << x) & (m2 - 1)] += p[s];
            }

            if a[i] >= m {
                dp[s] += p[s] * (a[i] - m);
            }
        }
    }

    let mut result = mint::from(0);

    for s in 0..m2 {
        // 10 が立っているパターンを足していく
        if s >> m & 1 > 0 {
            result += dp[s];
        }
    }

    println!("{}", result);
}
