use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    if x <= a {
        println!("1");
    } else if x > b {
        println!("0");
    } else {
        let p = c as f64 / (b as f64 - a as f64);
        println!("{}", p);
    }
}
