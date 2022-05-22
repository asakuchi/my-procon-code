use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        mut a: [usize; n],
    }

    let a_sum: usize = a.iter().sum();

    let mut priority_queue = BinaryHeap::new();

    for &num in &a {
        priority_queue.push(Reverse(num));
    }

    if l != a_sum {
        priority_queue.push(Reverse(l - a_sum));
    }

    let mut cost = 0;

    while let Some(Reverse(value)) = priority_queue.pop() {
        if let Some(Reverse(next)) = priority_queue.pop() {
            cost += value + next;

            priority_queue.push(Reverse(value + next));
        } else {
            break;
        }
    }

    println!("{}", cost);
}
