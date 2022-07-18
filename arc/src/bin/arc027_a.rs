use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        m: usize,
    }

    let mut result = 0;

    result += (18 - h - 1) * 60;
    result += 60 - m;

    println!("{}", result);
}
