use proconio::fastout;
use proconio::input;

const MAX_SIZE: usize = 1_000_000;
const MODULO: isize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let mut fac = vec![0; MAX_SIZE];
    let mut finv = vec![0; MAX_SIZE];

    combination_init(&mut fac, &mut finv);

    for a in 0..=x {
        let b = x - a;

        if b % 2 == 1 {
            continue;
        }

        let b = b / 2;

        if 2 * a + b == y {
            let count = combination((a + b) as isize, a as isize, &fac, &finv);
            println!("{}", count);
            return;
        }
    }

    println!("0");
}

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
