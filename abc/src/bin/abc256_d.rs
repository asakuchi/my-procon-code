use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort();

    lr.push((2_000_000_000, 2_000_000_000));

    let mut left = lr[0].0;
    let mut right = lr[0].1;

    for &(l, r) in &lr {
        if right < l {
            println!("{} {}", left, right);

            left = l;
            right = r;

            continue;
        }

        if r > right {
            right = r;
        }
    }
}
