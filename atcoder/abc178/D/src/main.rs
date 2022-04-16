use proconio::fastout;
use proconio::input;

const MODULO: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: usize,
    }

    let mut memo = vec![-1; s + 1];

    let result = rec(s, &mut memo);

    println!("{}", result);
}

fn rec(n: usize, memo: &mut Vec<isize>) -> usize {
    if n < 3 {
        return 0;
    }

    if n == 3 {
        return 1;
    }

    if memo[n] != -1 {
        return memo[n] as usize;
    }

    let mut result = 1;

    for i in 3..n {
        result += rec(n - i, memo);
        result %= MODULO;
    }

    memo[n] = result as isize;

    result
}
