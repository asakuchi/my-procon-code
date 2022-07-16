use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let p1_x = a * (2. * std::f64::consts::PI * (h + m / 60.) / 12.).sin();
    let p1_y = a * (2. * std::f64::consts::PI * (h + m / 60.) / 12.).cos();
    let p2_x = b * (2. * std::f64::consts::PI * m / 60.).sin();
    let p2_y = b * (2. * std::f64::consts::PI * m / 60.).cos();

    let result = ((p1_x - p2_x).powf(2.) + (p1_y - p2_y).powf(2.)).sqrt();

    println!("{}", result);
}
