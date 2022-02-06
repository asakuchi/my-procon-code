use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut h: isize,
        n: usize,
        a: [isize; n],
    }

    for value in a {
        h -= value;
    }

    println!("{}", if h <= 0 { "Yes" } else { "No" });
}
