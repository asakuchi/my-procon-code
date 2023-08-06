use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        r_c: [(isize, isize); n-1],
    }

    let mut r_set = HashSet::new();
    let mut c_set = HashSet::new();
    let mut naname_1 = HashSet::new();
    let mut naname_2 = HashSet::new();

    for &(r, c) in &r_c {
        r_set.insert(r);
        c_set.insert(c);
        naname_1.insert(r + c);
        naname_2.insert(r - c);
    }

    for i in 1..=n as isize {
        for j in 1..=n as isize {
            if !r_set.contains(&i)
                && !c_set.contains(&j)
                && !naname_1.contains(&(i + j))
                && !naname_2.contains(&(i - j))
            {
                println!("{} {}", i, j);
                return;
            }
        }
    }

    println!("-1");
}
