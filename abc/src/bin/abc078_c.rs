use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let p = (1f64 / 2.).powf(m as f64);

    let result = ((n - m) * 100 + m * 1_900) as f64 * (1. / p);

    println!("{}", result);
}
