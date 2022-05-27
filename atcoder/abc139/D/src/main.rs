use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let result: usize = (1..n).sum();

    println!("{}", result);
}
