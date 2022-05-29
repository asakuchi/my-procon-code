use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
    }

    let mut result = 0;

    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
