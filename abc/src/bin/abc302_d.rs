use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: isize,
        mut a: [isize; n],
        mut b: [isize; m],
    }

    a.sort();
    b.sort();

    let mut b_set = BTreeSet::new();

    for &x in b.iter() {
        b_set.insert(x);
    }

    let mut result: Option<isize> = None;

    for i in 0..n {
        let x = a[i];

        if let Some(&value) = b_set.range(x - d..=x + d).last() {
            if let Some(max_value) = result {
                result = Some(max_value.max(x + value));
            } else {
                result = Some(x + value);
            }
        }
    }

    if let Some(value) = result {
        println!("{}", value);
    } else {
        println!("-1");
    }
}
