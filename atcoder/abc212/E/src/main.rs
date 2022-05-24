use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const MODULO: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &uv {
        list[u].push(v);
        list[v].push(u);
    }

    let mut dp = vec![vec![0; n + 10]; k + 10];

    dp[0][0] = 1;

    for day in 0..k {
        let mut total = 0;

        for city in 0..n {
            total += dp[day][city];
        }

        for next in 0..n {
            dp[day + 1][next] = total - dp[day][next];
            dp[day + 1][next] %= MODULO;

            for &city in &list[next] {
                dp[day + 1][next] += MODULO;
                dp[day + 1][next] -= dp[day][city];
                dp[day + 1][next] %= MODULO;
            }
        }
    }

    println!("{}", dp[k][0]);
}
