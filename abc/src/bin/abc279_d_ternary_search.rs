//!
//! 三分探索
//!

use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let mut low = 0;
    let mut high = 1_000_000_000_000_000_000;

    while high - low > 3 {
        let c1 = (low * 2 + high) / 3;
        let c2 = (low + high * 2) / 3;

        if fall_time(a, b, c1) > fall_time(a, b, c2) {
            low = c1;
        } else {
            high = c2
        }
    }

    let mut result = low;
    let mut min_time = fall_time(a, b, low);

    for i in low..=high {
        let candi = fall_time(a, b, i);

        if candi < min_time {
            min_time = candi;
            result = i;
        }
    }

    println!("{:.8}", fall_time(a, b, result));
}

fn fall_time(a: f64, b: f64, time: usize) -> f64 {
    let g = (time + 1) as f64;

    a / g.sqrt() + b * time as f64
}
