use std::collections::BTreeSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
        q: usize,
        c_x: [(usize, usize); q],
    }

    let mut set = BTreeSet::new();

    // 番兵
    set.insert(0);
    set.insert(l);

    for &(c, x) in &c_x {
        if c == 1 {
            set.insert(x);
        } else {
            let &prev = set.range(..x).last().unwrap();
            let &next = set.range(x..).next().unwrap();

            println!("{}", next - prev);
        }
    }
}
