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

            if list[i].0 < list[j].0 {
                continue;
            }

            let mut ok = true;

            for &f_i in list[i].1.iter() {
                if !list[j].1.contains(&f_i) {
                    ok = false;
                    break;
                }
            }

            if !ok {
                continue;
            }

            let mut found = false;

            for &f_j in list[j].1.iter() {
                if !list[i].1.contains(&f_j) {
                    found = true;
                    break;
                }
            }

            if list[i].0 > list[j].0 || found {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
