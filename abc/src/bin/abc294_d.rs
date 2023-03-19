use std::collections::{BTreeSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut queue = VecDeque::new();

    for i in 1..=n {
        queue.push_back(i);
    }

    let mut set = BTreeSet::new();

    for _ in 0..q {
        input! {
            event: usize,
        }

        if event == 1 {
            let x = queue.pop_front().unwrap();

            set.insert(x);
        } else if event == 2 {
            input! {
                x: usize,
            }

            set.remove(&x);
        } else {
            let x = set.iter().next().unwrap();

            println!("{}", x);
        }
    }
}
