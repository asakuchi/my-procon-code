use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", 2 * n + 3);
}
