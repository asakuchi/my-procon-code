use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        c: char,
    }

    if c == 'O' || c == 'P' || c == 'K' || c == 'L' {
        println!("Right");
    } else {
        println!("Left");
    }
}
