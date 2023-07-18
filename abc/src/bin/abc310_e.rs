use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let a: Vec<_> = s
        .into_iter()
        .map(|c| if c == '1' { 1 } else { 0 })
        .collect();

    // dp[j][k]
    // f(i, j) = k になるiの個数(1<=i<=j)

    let mut dp = vec![vec![0; 2]; n + 1];

    for j in 1..=n {
        dp[j][nand(a[j - 1], 0)] += dp[j - 1][0];
        dp[j][nand(a[j - 1], 1)] += dp[j - 1][1];

        dp[j][a[j - 1]] += 1;
    }

    let mut result = 0_usize;

    // dp[j][1] というのは f(1, j) ~ f(j, j) のうち1のもののの和なので、
    // dp[1][1] ~ dp[n][1] を足せば求めたいものが出せる

    for j in 1..=n {
        result += dp[j][1];
    }

    println!("{}", result);
}

fn nand(i: usize, j: usize) -> usize {
    match i + j {
        2 => 0,
        _ => 1,
    }
}
