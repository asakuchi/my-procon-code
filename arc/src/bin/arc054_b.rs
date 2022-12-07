use proconio::input;

fn main() {
    input! {
        p: f64
    }

    let mut low = 0.;
    let mut high = 100.;

    for _ in 0..100 {
        let c1 = (low * 2. + high) / 3.;
        let c2 = (low + high * 2.) / 3.;

        if calc_time(c1, p) > calc_time(c2, p) {
            low = c1;
        } else {
            high = c2
        }
    }

    println!("{:.10}", calc_time(low, p));
}

fn calc_time(x: f64, p: f64) -> f64 {
    x + p / 2f64.powf(x / 1.5)
}
