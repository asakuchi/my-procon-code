use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        s: Chars,
    }

    let atcoder: Vec<_> = "atcoder".chars().collect();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), s.clone()));

    let mut visited = HashSet::new();

    visited.insert(s.clone());

    while let Some((Reverse(count), list)) = queue.pop() {
        if list == atcoder {
            println!("{}", count);
            return;
        }

        for i in 0..6 {
            let mut new_list = list.clone();
            new_list.swap(i, i + 1);

            if visited.contains(&new_list) {
                continue;
            }

            visited.insert(new_list.clone());

            queue.push((Reverse(count + 1), new_list));
        }
    }

    panic!("not found");
}
