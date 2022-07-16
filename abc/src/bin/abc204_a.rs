use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }

    if x != y {
        println!("{}", 3 - x - y);
    } else {
        println!("{}", x);
    }
}
