use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let mut sorted = x.clone();
    sorted.sort();

    for i in 0..n {
        if x[i] < sorted[n / 2] {
            println!("{}", sorted[n / 2]);
        } else {
            println!("{}", sorted[n / 2 - 1]);
        }
    }
}
