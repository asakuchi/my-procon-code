use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
    }

    println!("{}", (a - b) / 3. + b);
}
