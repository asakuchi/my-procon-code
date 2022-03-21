use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        t: usize,
        l: usize,
        x: usize,
        y: usize,
        q: usize,
        e: [usize; q],
    }

    for time in e.iter() {
        let time = time % t;

        let rate = (time as f64) / t as f64;

        let position = (
            0,
            -(l as f64 / 2.) * (2. * std::f64::consts::PI * rate).sin(),
            -(l as f64 / 2.) * (2. * std::f64::consts::PI * rate).cos() + (l as f64 / 2.),
        );

        let distance = x.pow(2) as f64 + (y as f64 - position.1).powf(2.);

        let angle = (position.2 / distance.sqrt()).atan() * 180. / std::f64::consts::PI;

        println!("{}", angle);
    }
}
