use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: isize,
        a: isize,
        b: isize,
    }

    if a >= b {
        println!("delicious");
    } else if x >= b - a {
        println!("safe");
    } else {
        println!("dangerous");
    }
}
