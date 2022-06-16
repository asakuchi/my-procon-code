use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    let mut i = 1;

    while i <= n {
        let x = n / i;
        let ni = n / x + 1;

        result += x * (ni - i);

        i = ni;
    }

    println!("{}", result);
}
