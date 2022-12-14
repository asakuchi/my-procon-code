use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    println!("{}", mod_comb_with_repetition(n, k));
}

fn mod_comb_with_repetition(n: usize, k: usize) -> usize {
    mod_comb(n - 1 + k, n - 1)
}

fn mod_pow(x: usize, a: usize) -> usize {
    if a == 1 {
        return x;
    }

    if a % 2 == 1 {
        return (x * mod_pow(x, a - 1)) % MOD;
    }

    let t = mod_pow(x, a / 2);

    return (t * t) % MOD;
}

fn mod_inv(x: usize) -> usize {
    return mod_pow(x, MOD - 2);
}

fn mod_perm(n: usize, k: usize) -> usize {
    let mut ret = 1;

    for i in 0..k {
        ret = (ret * (n - i)) % MOD;
    }

    return ret;
}

fn mod_comb(n: usize, k: usize) -> usize {
    let a = mod_perm(n, k);
    let b = mod_perm(k, k);

    return (a * mod_inv(b)) % MOD;
}
