use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }

    let sum: usize = t.iter().sum();

    let mut memo = vec![vec![-1; 1_000_001]; 101];

    let result = rec(n, (sum + 1) / 2, &t, 0, 0, &mut memo);

    println!("{}", result);
}

fn rec(
    n: usize,
    border: usize,
    t: &Vec<usize>,
    current: usize,
    total: usize,
    memo: &mut Vec<Vec<isize>>,
) -> usize {
    if memo[current][total] != -1 {
        return memo[current][total] as usize;
    }

    if current >= n {
        if total >= border {
            return total;
        } else {
            return border * 2; // NG
        }
    }

    let result_1 = rec(n, border, t, current + 1, total + t[current], memo);
    let result_2 = rec(n, border, t, current + 1, total, memo);

    let result = result_1.min(result_2);

    memo[current][total] = result as isize;

    result
}
