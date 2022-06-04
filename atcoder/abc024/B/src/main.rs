use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    }

    let mut result = 0;

    for i in 1..n {
        result += (a[i] - a[i - 1]).min(t);
    }

    // n人目の分
    result += t;

    println!("{}", result);
}
