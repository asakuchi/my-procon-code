//
// 解説AC
//

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut c = Vec::new();
    let mut p = Vec::new();
    let mut s = Vec::new();

    for _ in 0..n {
        input! {
            c_1: usize,
            p_1: usize,
            s_1: [usize; p_1],
        }

        let mut zero_count = 0;

        let mut s_2 = Vec::new();

        for &x in &s_1 {
            if x == 0 {
                zero_count += 1;
            } else {
                s_2.push(x);
            }
        }

        c.push(c_1 as f64 * p_1 as f64 / (p_1 as f64 - zero_count as f64));
        p.push(p_1 - zero_count);
        s.push(s_2);
    }

    // dp
    // ポイントがSからM以上にするために必要な金額の期待値Es
    // ポイントがM以上の状態では期待値0

    let mut dp = vec![0.; m + 110];

    for i in (0..m).rev() {
        let mut min_value = f64::MAX;

        for j in 0..n {
            let mut score = 0.;

            for &k in s[j].iter() {
                score += (dp[i + k] + c[j] as f64) / p[j] as f64;
            }

            min_value = min_value.min(score);
        }

        dp[i] = min_value;
    }

    println!("{}", dp[0]);
}
