use std::io;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let m: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    // ------------------------------------

    let result = power(m, n, MODULO);

    println!("{}", result);
}

const MODULO: usize = 1_000_000_007;

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
