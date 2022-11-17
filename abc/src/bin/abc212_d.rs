use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize
    }

    let mut priority_queue = BinaryHeap::new();

    let mut offset = 0;

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                x: isize,
            }

            priority_queue.push(Reverse(x - offset));
        } else if query == 2 {
            input! {
                x: isize,
            }

            offset += x;
        } else {
            if let Some(Reverse(x)) = priority_queue.pop() {
                println!("{}", x + offset);
            } else {
                panic!();
            }
        }
    }
}
