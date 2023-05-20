use std::collections::HashSet;

use proconio::{input, marker::Usize1};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut list = vec![HashSet::new(); n];

    let mut isolation = n;

    for _ in 0..q {
        input! {
            c: usize,
        }

        if c == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            }

            if list[u].len() == 0 {
                isolation -= 1;
            }

            if list[v].len() == 0 {
                isolation -= 1;
            }

            list[u].insert(v);
            list[v].insert(u);
        } else {
            input! {
                v: Usize1,
            }

            for &next in list[v].clone().iter() {
                list[next].remove(&v);

                if list[next].len() == 0 {
                    isolation += 1;
                }
            }

            if list[v].len() != 0 {
                isolation += 1;
            }

            list[v] = HashSet::new();
        }

        println!("{}", isolation);
    }
}
