use proconio::fastout;
use proconio::input;

const MODULO: isize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut result = 0;
    let mut sum = 0;
    let mut square = 0;

    for i in 0..n {
        sum += a[i];
        sum %= MODULO;

        square += a[i] * a[i];
        square %= MODULO;
    }

    for i in 0..n {
        result += sum * a[i];
        result %= MODULO;
    }

    result += MODULO;
    result -= square;
    result %= MODULO;

    result = result * mod_inverse(2, MODULO) % MODULO;

    println!("{}", result);
}

fn mod_inverse(a: isize, m: isize) -> isize {
    let mut a = a;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;

    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }

    u %= m;

    if u < 0 {
        u += m;
    }

    u
}
