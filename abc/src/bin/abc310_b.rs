use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: usize,
    }

    let mut list = Vec::new();

    for _ in 0..n {
        input! {
            p: usize,
            f: [usize],
        }

        let mut set = HashSet::new();

        for x in f {
            set.insert(x);
        }

        list.push((p, set));
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let (price_i, func_i) = &list[i];
            let (price_j, func_j) = &list[j];

            if !(price_i >= price_j && func_j.is_superset(func_i)) {
                continue;
            }

            if price_i > price_j || func_i.len() < func_j.len() {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
