use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut result = 0;

    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            result += 1;
        }
    }

    println!("{}", result);
}
