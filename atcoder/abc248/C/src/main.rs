use proconio::fastout;
use proconio::input;

const MODULO: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut memo = vec![vec![-1; k + 1]; n + 1];

    let result = rec(n, m, k, &mut memo);

    println!("{}", result);
}

fn rec(n: usize, m: usize, k: usize, memo: &mut Vec<Vec<isize>>) -> usize {
    if n == 0 {
        return 1;
    }

    if memo[n][k] != -1 {
        return memo[n][k] as usize;
    }

    let mut result = 0;

    for num in 1..=m {
        if k >= num {
            result += rec(n - 1, m, k - num, memo);
            result %= MODULO;
        }
    }

    result %= MODULO;

    memo[n][k] = result as isize;

    result
}
