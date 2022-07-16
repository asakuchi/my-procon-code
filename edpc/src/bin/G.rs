use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut xy: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(x, y) in &xy {
        list[x].push(y);
    }

    let mut result = 0;

    let mut memo = vec![-1; n];

    for i in 0..n {
        let next_result = rec(n, &list, i, &mut memo);

        result = result.max(next_result);
    }

    println!("{}", result);
}

fn rec(n: usize, list: &Vec<Vec<usize>>, current: usize, memo: &mut Vec<isize>) -> usize {
    if memo[current] != -1 {
        return memo[current] as usize;
    }

    let mut result = 0;

    for &next in &list[current] {
        let next_result = rec(n, list, next, memo) + 1;

        result = result.max(next_result);
    }

    memo[current] = result as isize;

    result
}
