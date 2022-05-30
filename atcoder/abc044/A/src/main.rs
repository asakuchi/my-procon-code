use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        y: usize,
    }

    let result;

    if n > k {
        result = k * x + (n - k) * y;
    } else {
        result = n * x;
    }

    println!("{}", result);
}
