use std::collections::VecDeque;

use proconio::input;

const INF: isize = 10_000_000_000_000;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let tail = a.len() - 1;

    let mut a = VecDeque::from(a);

    let mut dp = vec![vec![vec![INF; n + 1]; n + 1]; 3];

    let result = rec(n, &mut a, 1, 0, tail, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    a: &mut VecDeque<isize>,
    player: usize,
    head: usize,
    tail: usize,
    dp: &mut Vec<Vec<Vec<isize>>>,
) -> isize {
    if a.len() == 0 {
        return 0;
    }

    if dp[player][head][tail] != INF {
        return dp[player][head][tail];
    }

    let score = if player == 1 {
        // 先頭
        let value = a.pop_front().unwrap();
        let score_1 = rec(n, a, 2, head + 1, tail, dp) + value;
        a.push_front(value);

        // 末尾
        let value = a.pop_back().unwrap();
        let score_2 = rec(n, a, 2, head, tail - 1, dp) + value;
        a.push_back(value);

        score_1.max(score_2)
    } else {
        // 先頭
        let value = a.pop_front().unwrap();
        let score_1 = rec(n, a, 1, head + 1, tail, dp) - value;
        a.push_front(value);

        // 末尾
        let value = a.pop_back().unwrap();
        let score_2 = rec(n, a, 1, head, tail - 1, dp) - value;
        a.push_back(value);

        score_1.min(score_2)
    };

    dp[player][head][tail] = score;

    score
}
