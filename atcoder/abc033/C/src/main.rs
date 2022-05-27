use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut result = 0;

    for product in s.split('+') {
        if !product.contains("0") {
            result += 1;
        }
    }

    println!("{}", result);
}
