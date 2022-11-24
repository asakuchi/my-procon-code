use std::collections::VecDeque;

use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize
    }

    let mut deque = VecDeque::with_capacity(q);
    let mut priority_queue = BinaryHeap::new();

    for _ in 0..q {
        input! {
            query: usize
        }

        if query == 1 {
            input! {
                x: usize
            }

            deque.push_back(x);
        } else if query == 2 {
            if let Some(Reverse(x)) = priority_queue.pop() {
                println!("{}", x);
            } else {
                let x = deque.pop_front().unwrap();
                println!("{}", x);
            }
        } else {
            // sort

            while let Some(x) = deque.pop_front() {
                priority_queue.push(Reverse(x));
            }
        }
    }
}
