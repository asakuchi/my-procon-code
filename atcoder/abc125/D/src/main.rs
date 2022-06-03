use proconio::input;

const INF: isize = -1_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut memo = vec![vec![INF - 1; 2]; n];

    let result_1 = rec(n, &a, 1, true, &mut memo) - a[0];
    let result_2 = rec(n, &a, 1, false, &mut memo) + a[0];

    println!("{}", result_1.max(result_2));
}

fn rec(n: usize, a: &Vec<isize>, current: usize, prev: bool, memo: &mut Vec<Vec<isize>>) -> isize {
    if current == n {
        return 0;
    }

    if memo[current][prev as usize] != INF - 1 {
        return memo[current][prev as usize];
    }

    let current_value = if prev { -a[current] } else { a[current] };

    // -1を乗算する
    // n-1の時はできない
    let score_1 = if current != n - 1 {
        rec(n, a, current + 1, true, memo) - current_value
    } else {
        INF
    };

    // -1を乗算しない
    let score_2 = rec(n, a, current + 1, false, memo) + current_value;

    let score = score_1.max(score_2);

    memo[current][prev as usize] = score;

    score
}
