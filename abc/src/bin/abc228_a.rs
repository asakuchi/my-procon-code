use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    }

    if s < t {
        if s <= x && x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if s <= x || x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
