//!
//! 任意点周りの回転移動（アフィン変換）
//! https://imagingsolution.net/math/rotate-around-point/
//!

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

    let center_x = (xn2 + x0) / 2.;
    let center_y = (yn2 + y0) / 2.;

    let angle = 2. * std::f64::consts::PI / n as f64;

    let x1 = x0 * angle.cos() - y0 * angle.sin() + center_x - center_x * angle.cos()
        + center_y * angle.sin();
    let y1 = x0 * angle.sin() + y0 * angle.cos() + center_y
        - center_x * angle.sin()
        - center_y * angle.cos();

    println!("{} {}", x1, y1);
}
