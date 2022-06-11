use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }

    let mut set = HashSet::new();

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

            let (x3, y3) = xy[j];

            let (x2, y2) = (x3, y1);
            let (x4, y4) = (x1, y3);

            if (x2, y2) == (x1, y1)
                || (x4, y4) == (x1, y1)
                || (x2, y2) == (x3, y3)
                || (x4, y4) == (x3, y3)
            {
                continue;
            }

            if set.contains(&(x2, y2)) && set.contains(&(x4, y4)) {
                result += 1;
            }
        }
    }

    println!("{}", result / 4);
}
