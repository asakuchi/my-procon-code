use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let x = a / (a.powf(2.) + b.powf(2.)).sqrt();
    let y = b / (a.powf(2.) + b.powf(2.)).sqrt();

    println!("{} {}", x, y);
}
