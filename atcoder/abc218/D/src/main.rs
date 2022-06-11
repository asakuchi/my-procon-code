use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }

    let mut set = BTreeSet::new();

    for &(x, y) in &xy {
        set.insert((x, y));
    }

    let mut result = 0;

    for i in 0..n {
        let (x1, y1) = xy[i];

        for j in 0..n {
            if i == j {
                continue;
            }

            let (x2, y2) = xy[j];

            if y1 != y2 {
                continue;
            }

            let x3 = x2;
            let x4 = x1;

            for &(_, y3) in set.range((x3, 0)..(x3 + 1, 0)) {
                if (x3, y3) == (x2, y2) {
                    continue;
                }

                let y4 = y3;

                if (x4, y4) == (x1, y1) {
                    continue;
                }

                if set.contains(&(x4, y4)) {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result / 4);
}
