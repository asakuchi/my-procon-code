use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a > b {
        println!("0");
    } else {
        println!("{}", b - a + 1);
    }
}
