use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut manhattan = 0;

    for i in 0..n {
        manhattan += a[i].abs();
    }

    let mut euclidean = 0;

    for i in 0..n {
        euclidean += a[i].pow(2);
    }

    let euclidean = (euclidean as f64).sqrt();

    let mut chebyshev = 0;

    for i in 0..n {
        chebyshev = chebyshev.max(a[i].abs());
    }

    println!("{}", manhattan);
    println!("{}", euclidean);
    println!("{}", chebyshev);
}
