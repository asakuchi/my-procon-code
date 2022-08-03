use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        v: [isize; n],
    }

    let mut v = VecDeque::from(v);

    let mut dp = vec![vec![-1; n + 1]; n + 1];

    let hands = BinaryHeap::new();

    let result = rec(n, k, &mut v, &hands, 0, n, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    k: usize,
    v: &mut VecDeque<isize>,
    hands: &BinaryHeap<Reverse<isize>>,
    left: usize,
    right: usize,
    dp: &mut Vec<Vec<isize>>,
) -> isize {
    if k == 0 {
        return 0;
    }

    if dp[left][right] != -1 {
        return dp[left][right];
    }

    let mut result = 0;

    // 操作A 左端の宝石
    if let Some(stone) = v.pop_front() {
        let mut new_hands = hands.clone();

        if stone < 0 {
            new_hands.push(Reverse(stone));
        }

        let score = rec(n, k - 1, v, &new_hands, left + 1, right, dp) + stone;

        result = result.max(score);

        v.push_front(stone);
    }

    // 操作B 右端の宝石
    if let Some(stone) = v.pop_back() {
        let mut new_hands = hands.clone();

        if stone < 0 {
            new_hands.push(Reverse(stone));
        }

        let score = rec(n, k - 1, v, &new_hands, left, right - 1, dp) + stone;

        result = result.max(score);

        v.push_back(stone);
    }

    let mut score = 0;
    let mut new_hands = hands.clone();

    for _ in 0..k {
        if let Some(Reverse(stone)) = new_hands.pop() {
            if stone < 0 {
                score -= stone
            }
        }
    }

    result = result.max(score);

    dp[left][right] = result;

    result
}
