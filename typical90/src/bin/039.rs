use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a_b: [(Usize1, Usize1); n - 1],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    let mut dp = vec![0; n];

    rec(n, &list, 0, 0, &mut dp);

    let mut result = 0;

    for i in 0..n {
        result += dp[i] * (n - dp[i]);
    }

    // println!("{:?}", dp);
    println!("{}", result);
}

fn rec(n: usize, list: &Vec<Vec<usize>>, current: usize, parent: usize, dp: &mut Vec<usize>) {
    dp[current] = 1;

    for &next in list[current].iter() {
        if next == parent {
            continue;
        }

        rec(n, list, next, current, dp);

        dp[current] += dp[next];
    }
}
