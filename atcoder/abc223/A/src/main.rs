use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    if x % 100 == 0 && x != 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
