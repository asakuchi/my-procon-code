use proconio::fastout;
use proconio::input;

const MODULO: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut memo = vec![vec![-1; 10]; n - 1];

    let mut result = 0;

    for i in 1..=9 {
        let count = rec(n, 0, i, &mut memo);
        result += count;
    }

    result %= MODULO;

    println!("{}", result);
}

fn rec(n: usize, index: usize, current: usize, memo: &mut Vec<Vec<isize>>) -> usize {
    if index == n - 1 {
        return 1;
    }

    if memo[index][current] != -1 {
        return memo[index][current] as usize;
    }

    let mut result = 0;

    if current != 1 {
        result += rec(n, index + 1, current - 1, memo) % MODULO;
    }

    result += rec(n, index + 1, current, memo) % MODULO;

    if current != 9 {
        result += rec(n, index + 1, current + 1, memo) % MODULO;
    }

    result %= MODULO;

    memo[index][current] = result as isize;

    result
}
