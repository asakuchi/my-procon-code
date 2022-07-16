use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let result = 2 * a + 100 - b;

    println!("{}", result);
}
