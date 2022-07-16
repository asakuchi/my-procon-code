use std::io;

const INF: usize = 100_000_000;

fn main() {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let mut c: Vec<usize> = Vec::with_capacity(m);

    for ci in iter {
        c.push(ci.parse().unwrap());
    }

    /* --------------------------------------------------- */

    let mut dp = vec![INF; 50_000 + 10_000 + 1];

    dp[0] = 0;

    for i in 0..m {
        for j in 0..=n {
            dp[j + c[i]] = std::cmp::min(dp[j + c[i]], dp[j] + 1);
        }
    }

    println!("{}", dp[n]);
}
