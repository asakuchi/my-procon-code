use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut set = HashSet::new();

    for value in &xy {
        set.insert(value);
    }

    let mut result = 0;

    for i in 0..n {
        for j in 0..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];

            let x3 = x2 - (y2 - y1);
            let y3 = y2 + (x2 - x1);
            let x4 = x3 - (x2 - x1);
            let y4 = y3 - (y2 - y1);

            if set.contains(&(x3, y3)) && set.contains(&(x4, y4)) {
                let edge_pow2 = (x1 - x2).pow(2) + (y1 - y2).pow(2);

                result = std::cmp::max(result, edge_pow2);
            }
        }
    }

    println!("{}", result);
}
