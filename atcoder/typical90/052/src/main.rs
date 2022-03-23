use proconio::fastout;
use proconio::input;

const MODULO: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    let mut result = 1;

    for i in 0..n {
        let mut dice_sum = 0;

        for j in 0..6 {
            dice_sum += a[i][j];
            dice_sum %= MODULO;
        }

        result *= dice_sum;
        result %= MODULO;
    }

    println!("{}", result);
}
