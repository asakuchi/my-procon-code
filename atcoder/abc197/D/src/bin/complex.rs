use num_complex::Complex;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        x0: f64,
        y0: f64,
        xn2: f64,
        yn2: f64,
    }

    let point_0 = Complex::new(x0, y0);
    let point_n2 = Complex::new(xn2, yn2);

    let center = (point_0 + point_n2) / 2.;

    let result = center
        + (point_0 - center) * Complex::from_polar(&1.0, &(std::f64::consts::PI * 2. / n as f64));

    println!("{} {}", result.re, result.im);
}
