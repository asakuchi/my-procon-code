use num_rational::Ratio;
use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

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

    let mut result_normal = HashSet::new();
    let mut result_x = HashSet::new();

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];

            if x2 == x1 {
                let mut count = 2;

                for l in j + 1..n {
                    if i == l || j == l {
                        continue;
                    }

                    let (x3, _y3) = xy[l];

                    if x3 == x1 {
                        count += 1;
                    }

                    if count >= k {
                        result_x.insert(x1);
                    }
                }
            } else {
                let mut count = 2;

                let a = Ratio::new_raw(y2 - y1, x2 - x1);
                let b = Ratio::new_raw(y1, 1) - a * x1;

                for l in j + 1..n {
                    if i == l || j == l {
                        continue;
                    }

                    let (x3, y3) = xy[l];

                    if Ratio::new_raw(y3, 1) == a * x3 + b {
                        count += 1;
                    }
                }

                if count >= k {
                    result_normal.insert((a, b));
                }
            }
        }
    }

    println!("{}", result_normal.len() + result_x.len());
}
