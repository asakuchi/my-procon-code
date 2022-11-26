use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let mut result = a;

    let x = 1. / ((2. * b / a).cbrt()).powf(2.) - 1.;

    for i in -100isize..100 {
        let count = x as isize + i;

        let count = if count < 0 {
            0 as usize
        } else {
            count as usize
        };

        let time = a / (count as f64 + 1.).sqrt();

        if time + b * (count as f64) < result {
            result = time + b * (count as f64);
        }
    }

    println!("{:.8}", result);
}
