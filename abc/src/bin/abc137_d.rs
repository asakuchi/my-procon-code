use std::{cmp::Reverse, collections::BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_b: [(usize, usize); n],
    }

    a_b.sort_by_key(|&(a, _b)| Reverse(a));

    let mut set = BTreeSet::new();

    let mut unique_key = 0;

    let mut result = 0;

    for day in 1..=m {
        while let Some((a, b)) = a_b.pop() {
            if day < a {
                a_b.push((a, b));
                break;
            }

            set.insert((b, unique_key));
            unique_key += 1;
        }

        if let Some(&(money, key)) = set.iter().last() {
            set.remove(&(money, key));

            // println!("day {} money {}", day, money);
            result += money;
        }
    }

    println!("{}", result);
}
