use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    }

    let pattern = vec![
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];

    let mut result = false;

    for &(i1, j1) in pattern.iter() {
        for &(i2, j2) in pattern.iter() {
            if x1 + i1 == x2 + i2 && y1 + j1 == y2 + j2 {
                result = true;
            }
        }
    }

    println!("{}", if result { "Yes" } else { "No" });
}
