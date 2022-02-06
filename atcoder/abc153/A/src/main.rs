use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        a: usize,
    }

    let mut count = h / a;

    if h % a != 0 {
        count += 1;
    }

    println!("{}", count);
}
