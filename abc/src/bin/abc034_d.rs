// use proconio::fastout;
use proconio::input;

// #[fastout]
fn main() {
    // let a = 2000000020;
    // let b = 20;

    // println!("{}", (a - b) % MODULO);
    // println!("{}", (a % MODULO - b % MODULO));
    // println!("{}", modulo(a % MODULO - b % MODULO, MODULO));

    // let a = 12345678900000;
    // let b = 100000;

    // // println!("{}", a * mod_inverse(b, MODULO) % MODULO);
    // println!("{}", a % MODULO * mod_inverse(b, MODULO) % MODULO);

    input! {
        w: isize,
        h: isize,
    }

    let mut fac = vec![0; MAX_SIZE];
    let mut finv = vec![0; MAX_SIZE];

    // (H+W-2) C (H-1)
    // (H + W - 2)! / (H - 1)!(W - 1)!
    combination_init(&mut fac, &mut finv);
    println!("{}", combination(h + w - 2, h - 1, &fac, &finv));
}

const MODULO: isize = 1_000_000_007;

///
/// 負の数にも対応した % 演算
///
fn modulo(value: isize, m: isize) -> isize {
    let mut result = value % m;

    if result < 0 {
        result += m;
    }

    result
}

// fn mod_inverse(a: isize, m: isize) -> isize {
//     let extgcd = isize::extended_gcd(&a, &m);

//     (m + extgcd.x % m) % m
// }

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

const MAX_SIZE: usize = 510000;

fn combination_init(fac: &mut Vec<isize>, finv: &mut Vec<isize>) {
    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;

    let mut inv = vec![0; MAX_SIZE];
    inv[1] = 1;

    for i in 2..MAX_SIZE {
        fac[i] = fac[i - 1] * i as isize % MODULO;
        inv[i] = MODULO - inv[MODULO as usize % i] * (MODULO / i as isize) % MODULO;
        finv[i] = finv[i - 1] * inv[i] % MODULO;
    }
}

fn combination(n: isize, k: isize, fac: &Vec<isize>, finv: &Vec<isize>) -> isize {
    if n < k {
        return 0;
    }
    if n < 0 || k < 0 {
        return 0;
    }

    fac[n as usize] * (finv[k as usize] * finv[(n - k) as usize] % MODULO) % MODULO
}
