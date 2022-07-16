// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize,isize);n],
    }

    let mut max = 0.0;

    for (x1, y1) in xy.iter() {
        for (x2, y2) in xy.iter() {
            let r = (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt();

            if r > max {
                max = r;
            }
        }
    }

    println!("{:?}", max);
}
