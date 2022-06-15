use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let price = (n as f64 * 1.08).floor() as u32;

    if price < 206 {
        println!("Yay!");
    } else if price > 206 {
        println!(":(");
    } else {
        println!("so-so");
    }
}
