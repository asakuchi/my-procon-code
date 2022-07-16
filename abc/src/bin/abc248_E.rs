use num_rational::Ratio;
use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut xy: [(isize, isize); n],
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut lines = HashMap::new();
    let mut lines_x = HashMap::new();

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];

            if x2 == x1 {
                *lines_x.entry(x1).or_insert(0) += 1;
            } else {
                let a = Ratio::new_raw(y2 - y1, x2 - x1);
                let b = Ratio::new_raw(y1, 1) - a * x1;

                *lines.entry((a, b)).or_insert(0) += 1;
            }
        }
    }

    let mut result = 0;

    for (_, &count) in lines.iter() {
        if count >= k * (k - 1) / 2 {
            result += 1;
        }
    }

    for (_, &count) in lines_x.iter() {
        if count >= k * (k - 1) / 2 {
            result += 1;
        }
    }

    println!("{}", result);
}
