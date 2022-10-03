use proconio::{input, marker::Usize1};

const INF: usize = 1_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a_b_c: [(Usize1, Usize1, usize); m],
        e: [Usize1; k],
    }

    let mut dp = vec![INF; n];

    dp[0] = 0;

    for i in 0..k {
        let load = e[i];

        let (a, b, c) = a_b_c[load];

        dp[b] = dp[b].min(dp[a] + c);
    }

    if dp[n - 1] == INF {
        println!("-1");
    } else {
        println!("{}", dp[n - 1]);
    }
}
