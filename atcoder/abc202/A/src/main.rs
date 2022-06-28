use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    println!("{}", 7 * 3 - a - b - c);
}
