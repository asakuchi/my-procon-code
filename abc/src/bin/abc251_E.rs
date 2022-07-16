use proconio::fastout;
use proconio::input;

const INF: usize = 1_000_000_000_000_000_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut memo = vec![vec![vec![-1; n]; 2]; 2];

    let select_first = rec(n, &a, 1, true, true, &mut memo) + a[0];
    let not_select_first = rec(n, &a, 1, false, false, &mut memo);

    println!("{}", select_first.min(not_select_first));
}

fn rec(
    n: usize,
    a: &Vec<usize>,
    current: usize,
    first_selected: bool,
    prev_selected: bool,
    memo: &mut Vec<Vec<Vec<isize>>>,
) -> usize {
    if current >= n {
        return 0;
    }

    if memo[first_selected as usize][prev_selected as usize][current] != -1 {
        return memo[first_selected as usize][prev_selected as usize][current] as usize;
    }

    let select_current = rec(n, a, current + 1, first_selected, true, memo) + a[current];

    let not_select_current = if prev_selected && (current != n - 1 || first_selected) {
        rec(n, a, current + 1, first_selected, false, memo)
    } else {
        // 前を選んでいなければ、今選ばなければいけない
        // 最初を選んでいなければ、最後は選ばなければいけない
        INF
    };

    let result = select_current.min(not_select_current);

    memo[first_selected as usize][prev_selected as usize][current] = result as isize;

    result
}
