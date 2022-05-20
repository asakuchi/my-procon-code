use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut result = 0;

    for i in 1..n {
        if a[i] < a[i - 1] {
            let diff = a[i - 1] - a[i];

            result += diff;
            a[i] += diff;
        }
    }

    println!("{}", result);
}
