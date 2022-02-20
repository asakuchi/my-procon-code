use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
    }

    if a - b == 1 || b - a == 1 || (a == 1 && b == 10) || (a == 10 && b == 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}
