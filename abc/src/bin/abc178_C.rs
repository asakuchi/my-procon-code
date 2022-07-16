use proconio::fastout;
use proconio::input;

const MODULO: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut memo = vec![vec![vec![-1; 2]; 2]; n];

    let result = rec(n, 0, false, false, &mut memo);

    println!("{}", result);
}

fn rec(
    n: usize,
    current: usize,
    has_zero: bool,
    has_nine: bool,
    memo: &mut Vec<Vec<Vec<isize>>>,
) -> usize {
    if current == n {
        if has_nine && has_zero {
            return 1;
        } else {
            return 0;
        }
    }

    if memo[current][has_zero as usize][has_nine as usize] != -1 {
        return memo[current][has_zero as usize][has_nine as usize] as usize;
    }

    let result_zero = rec(n, current + 1, true, has_nine, memo) * 1 % MODULO;
    let result_nine = rec(n, current + 1, has_zero, true, memo) * 1 % MODULO;
    let result_other = rec(n, current + 1, has_zero, has_nine, memo) * 8 % MODULO;

    let result = (result_zero + result_nine + result_other) % MODULO;

    memo[current][has_zero as usize][has_nine as usize] = result as isize;

    result
}
