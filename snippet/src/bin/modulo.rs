fn main() {
    let mut fac = vec![0; MAX_SIZE];
    let mut finv = vec![0; MAX_SIZE];

    let n = 4;
    let k = 2;

    // nCk
    combination_init(&mut fac, &mut finv);
    println!("{}", combination(n, k, &fac, &finv));
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
