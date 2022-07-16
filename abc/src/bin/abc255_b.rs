use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        mut xy: [(isize, isize); n],
    }

    let mut list = vec![false; n];

    for i in 0..k {
        list[a[i]] = true;
    }

    let mut has_light = Vec::new();
    let mut no_light = Vec::new();

    for i in 0..n {
        if list[i] {
            has_light.push(xy[i]);
        } else {
            no_light.push(xy[i]);
        }
    }

    // no_light からの最小値の最大値

    let mut result = 0.;

    for &(x2, y2) in no_light.iter() {
        let mut score = 1_000_000_000.;

        for &(x1, y1) in has_light.iter() {
            let len = ((x2 - x1).pow(2) as f64 + (y2 - y1).pow(2) as f64).sqrt();

            if len < score {
                score = len;
            }
        }

        if result < score {
            result = score;
        }
    }

    println!("{}", result);
}
