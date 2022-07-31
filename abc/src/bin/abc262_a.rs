use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        y: usize,
    }

    let diff = y % 4;

    if diff == 0 {
        println!("{}", y + 2);
    } else if diff == 1 {
        println!("{}", y + 1);
    } else if diff == 2 {
        println!("{}", y);
    } else {
        println!("{}", y + 3);
    }
}
