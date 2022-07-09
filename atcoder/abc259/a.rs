use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        t: usize,
        d: usize,
    }

    if m > x {
        println!("{}", t);
    } else {
        println!("{}", t - (x - m) * d);
    }
}
