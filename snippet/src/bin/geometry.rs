//!
//! 幾何
//!

fn main() {
    println!("{}", cos_formula(3., 4., 0.5 * std::f64::consts::PI,));

    println!("{}", distance(0., 4., -3., 0.));

    println!("{:?}", rotate(1., 2., 3., 4., 0.5 * std::f64::consts::PI));
}

///
/// 余弦定理
///
/// ```
/// assert_eq!(cos_formula(3., 4., 0.5 * std::f64::consts::PI,), 5);
/// ```
///
fn cos_formula(a: f64, b: f64, c_angle_radian: f64) -> f64 {
    (a * a + b * b - 2. * a * b * c_angle_radian.cos()).sqrt()
}

///
/// 2点間の距離
///
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powf(2.) + (y1 - y2).powf(2.)).sqrt()
}

///
/// 任意点周りの回転移動（アフィン変換）
/// https://imagingsolution.net/math/rotate-around-point/
///
fn rotate(x: f64, y: f64, center_x: f64, center_y: f64, angle_radian: f64) -> (f64, f64) {
    let rotated_x = x * angle_radian.cos() - y * angle_radian.sin() + center_x
        - center_x * angle_radian.cos()
        + center_y * angle_radian.sin();
    let rotated_y = x * angle_radian.sin() + y * angle_radian.cos() + center_y
        - center_x * angle_radian.sin()
        - center_y * angle_radian.cos();

    (rotated_x, rotated_y)
}
