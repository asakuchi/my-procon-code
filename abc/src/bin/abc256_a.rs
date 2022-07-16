use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: u32,
    }

    println!("{}", 2usize.pow(n));
}
