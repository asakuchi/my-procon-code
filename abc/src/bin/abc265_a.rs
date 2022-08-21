use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }

    let mut result = 1_000_000_000;

    for i in 0..=n {
        if (n - i) % 3 == 0 {
            let j = (n - i) / 3;

            let price = i * x + j * y;

            result = result.min(price);
        }
    }
    println!("{}", result);
}
