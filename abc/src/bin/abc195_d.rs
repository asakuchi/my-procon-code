use std::{cmp::Reverse, collections::BTreeMap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut w_v: [(usize, usize); n],
        x: [usize; m],
        query: [(Usize1, Usize1); q],
    }

    w_v.sort_by_key(|x| Reverse(x.1));

    for (l, r) in query {
        let mut score = 0;

        let mut boxes = BTreeMap::new();

        for i in 0..l {
            *boxes.entry(x[i]).or_insert(0) += 1;
        }

        for i in r + 1..m {
            *boxes.entry(x[i]).or_insert(0) += 1;
        }

        for &(w, v) in &w_v {
            let one_box = boxes.range(w..).next();

            if let Some((&size, &count)) = one_box {
                score += v;

                if count == 1 {
                    boxes.remove(&size);
                } else {
                    *boxes.entry(size).or_insert(0) = count - 1;
                }
            }
        }

        println!("{}", score);
    }
}
