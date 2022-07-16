use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
    }

    println!("{}", a / 100. * b);
}
