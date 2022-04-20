use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    let mut right = 1;

    for left in 0..n {
        while right < n && (right <= left || a[right - 1] < a[right]) {
            right += 1;
        }

        result += right - left;
    }

    println!("{}", result);
}
