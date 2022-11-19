use std::collections::HashSet;

use proconio::fastout;
use proconio::{input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        _n: usize,
        q: usize,
        t_a_b: [(usize, Usize1, Usize1); q],
    }

    let mut set = HashSet::new();

    for &(t, a, b) in &t_a_b {
        if t == 1 {
            set.insert((a, b));
        } else if t == 2 {
            set.remove(&(a, b));
        } else {
            if set.contains(&(a, b)) && set.contains(&(b, a)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
