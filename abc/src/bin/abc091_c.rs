use std::collections::{BTreeSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a_b: [(usize, usize); n],
        mut c_d: [(usize, usize); n],
    }

    let mut y_set = BTreeSet::new();

    a_b.sort_by(|a, b| a.0.cmp(&b.0));
    c_d.sort_by(|a, b| a.0.cmp(&b.0));
    let mut a_b = VecDeque::from(a_b);

    let mut result = 0;

    for (c, d) in c_d {
        while let Some((a, b)) = a_b.pop_front() {
            if a < c {
                y_set.insert(b);
            } else {
                a_b.push_front((a, b));
                break;
            }
        }

        if let Some(&b) = y_set.range(..d).last() {
            result += 1;
            y_set.remove(&b);
        }
    }

    println!("{}", result);
}
