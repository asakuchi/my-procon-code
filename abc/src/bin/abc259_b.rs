use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }

    let rad = d * std::f64::consts::PI / 180.;

    let x = a * rad.cos() - b * rad.sin();
    let y = a * rad.sin() + b * rad.cos();

    println!("{} {}", x, y);
}
