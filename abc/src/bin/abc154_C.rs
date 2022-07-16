use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut s = vec![0.; n + 1];

    for i in 0..n {
        let kitaiti = (1 + p[i]) as f64 / 2.;

        s[i + 1] = s[i] + kitaiti;
    }

    let mut max = 0.;

    for i in k..n + 1 {
        // println!("{}", s[i] - s[i - k]);
        if s[i] - s[i - k] > max {
            max = s[i] - s[i - k];
        }
    }

    println!("{}", max);
}
