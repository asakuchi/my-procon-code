use itertools::*;
use proconio::marker::*;
use proconio::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(Usize1, Usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for (i, (a, b, c)) in e.into_iter().enumerate() {
        g[a].push((b, c, i));
        g[b].push((a, c, i));
    }
    let inf = std::i64::MAX / 2;
    let mut dp = vec![(inf, m); n];
    dp[0] = (0, m);
    let mut h = std::collections::BinaryHeap::new();
    h.push((0, 0));
    while let Some((d, v)) = h.pop() {
        let d = -d;
        if d > dp[v].0 {
            continue;
        }
        for &(u, w, k) in g[v].iter() {
            let d = d + w;
            if dp[u] > (d, k) {
                dp[u] = (d, k);
                h.push((-d, u));
            }
        }
    }
    println!("{}", dp[1..].iter().map(|p| p.1 + 1).join(" "));
}
