fn main() {
    let x = 83;
    let n = 999999570;

    let result = power(x, n, MODULO);

    println!("{}", result);
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
