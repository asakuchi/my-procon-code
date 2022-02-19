use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: i128,
    }

    if x % 10 != 0 && x < 0 {
        println!("{}", (x / 10) - 1);
    } else {
        println!("{}", (x / 10));
    }
}
