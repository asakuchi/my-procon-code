use std::collections::{HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        init_a: usize,
        init_b: usize,
        init_c: usize,
    }

    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((init_a, init_b, init_c, 0));
    visited.insert((init_a, init_b, init_c));

    while let Some((a, b, c, count)) = queue.pop_front() {
        // 終了条件
        if a == b && b == c {
            println!("{}", count);
            return;
        }

        // a,b を1増やす
        if !visited.contains(&(a + 1, b + 1, c)) {
            visited.insert((a + 1, b + 1, c));
            queue.push_back((a + 1, b + 1, c, count + 1));
        }

        // b,c を1増やす
        if !visited.contains(&(a, b + 1, c + 1)) {
            visited.insert((a, b + 1, c + 1));
            queue.push_back((a, b + 1, c + 1, count + 1));
        }

        // c,a を1増やす
        if !visited.contains(&(a + 1, b, c + 1)) {
            visited.insert((a + 1, b, c + 1));
            queue.push_back((a + 1, b, c + 1, count + 1));
        }

        // a を2増やす
        if !visited.contains(&(a + 2, b, c)) {
            visited.insert((a + 2, b, c));
            queue.push_back((a + 2, b, c, count + 1));
        }

        // b を2増やす
        if !visited.contains(&(a, b + 2, c)) {
            visited.insert((a, b + 2, c));
            queue.push_back((a, b + 2, c, count + 1));
        }

        // c を2増やす
        if !visited.contains(&(a, b, c + 2)) {
            visited.insert((a, b, c + 2));
            queue.push_back((a, b, c + 2, count + 1));
        }
    }
}
