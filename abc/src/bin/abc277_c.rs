use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n],
    }

    let mut map = HashMap::new();

    for &(a, b) in &a_b {
        let list = map.entry(a).or_insert(Vec::new());
        list.push(b);

        let list = map.entry(b).or_insert(Vec::new());
        list.push(a);
    }

    let mut visited = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(1);
    visited.entry(1).or_insert(true);

    let mut result = 1;

    while let Some(current) = queue.pop_front() {
        if let Some(list) = map.get(&current) {
            for &next in list {
                if let Some(&value) = visited.get(&next) {
                    if value {
                        continue;
                    }
                }

                result = result.max(next);

                visited.insert(next, true);

                queue.push_back(next);
            }
        }
    }

    println!("{}", result);
}
