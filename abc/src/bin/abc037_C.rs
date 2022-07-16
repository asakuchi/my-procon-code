use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut result = 0;
    let mut k_sum = 0;

    for i in 0..k {
        k_sum += a[i];
    }

    result += k_sum;

    for i in k..n {
        k_sum -= a[i - k];
        k_sum += a[i];

        result += k_sum;
    }

    println!("{}", result);
}
