use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut result = t.clone();

    for i in 0..3 * n {
        let current = i % n;

        let prev = if current > 0 { current - 1 } else { n - 1 };

        result[current] = t[current].min(result[prev] + s[prev]);
    }

    for i in 0..n {
        println!("{}", result[i]);
    }
}
