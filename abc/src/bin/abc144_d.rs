use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }

    let half_theta = std::f64::consts::PI / 2. - (a / b).atan();

    let half_x = a * a * b / 2.;

    if x == half_x {
        println!("{}", half_theta * 180. / std::f64::consts::PI);
    } else if x > half_x {
        // x = a * a * (b - a * 1/tan(θ)) + a * a * a /tan(θ) / 2
        let theta = std::f64::consts::PI / 2. - (a * a * a / (2. * a * a * b - 2. * x)).atan();
        println!("{}", theta * 180. / std::f64::consts::PI);
    } else {
        // x = a * b * b * tan(PI / 2 - θ) / 2
        let theta = std::f64::consts::PI / 2. - (2. * x / (a * b * b)).atan();
        println!("{}", theta * 180. / std::f64::consts::PI);
    }
}
