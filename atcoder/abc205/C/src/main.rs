use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: usize,
    }

    if c % 2 == 0 {
        if a.abs() > b.abs() {
            println!(">");
        } else if a.abs() < b.abs() {
            println!("<");
        } else {
            println!("=");
        }
    } else {
        if a > b {
            println!(">");
        } else if a < b {
            println!("<");
        } else {
            println!("=");
        }
    }
}
