use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,

    }

    let result = ((h as f64) * (12800000. + h as f64)).powf(1. / 2.);
    println!("{}", result);
}
