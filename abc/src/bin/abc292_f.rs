use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let (a, b) = if a < b { (a, b) } else { (b, a) };

    let mut low = 0.;
    let mut high = 30.;

    // 実数で探索する場合の終了条件
    for _ in 0..1_000 {
        // while high - low > 3 {
        let c1 = (low * 2. + high) / 3.;
        let c2 = (low + high * 2.) / 3.;

        if calc(c1, a, b) < calc(c2, a, b) {
            low = c1;
        } else {
            high = c2
        }

        // println!("{}", low);
    }

    println!("{}", calc(low, a, b));
}

fn calc(angle: f64, a: usize, b: usize) -> f64 {
    let theta = angle / 360. * 2. * PI;

    let one = ((a as f64) / (theta + PI / 3.).sin()).abs();
    let two = ((b as f64) / theta.cos()).abs();

    if one < two {
        one
    } else {
        two
    }
}
