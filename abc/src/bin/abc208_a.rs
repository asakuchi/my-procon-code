use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a <= b && b <= 6 * a {
        println!("Yes");
    } else {
        println!("No");
    }
}
