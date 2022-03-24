use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if n == 1 {
        println!("{}", k);
    } else if k == 1 {
        println!("{}", 0);
    } else {
        println!(
            "{}",
            power(k - 2, n - 2, MODULO) % MODULO * (k - 1) % MODULO * k % MODULO
        );
    }
}

const MODULO: usize = 1_000_000_007;

///
/// 繰り返し二乗法
///
fn power(x: usize, n: usize, modulo: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut result = power(x * x % modulo, n / 2, modulo);

    if n % 2 != 0 {
        result = result * x % modulo;
    }

    result
}
