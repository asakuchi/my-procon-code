use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    // 二項係数の和は2^n
    let all = mod_pow(2, n);

    let a_count = mod_comb(n, a);
    let b_count = mod_comb(n, b);

    let mut result = all;
    result += MOD as usize;
    result -= a_count as usize;
    result %= MOD as usize;

    result += MOD as usize;
    result -= b_count as usize;
    result %= MOD as usize;

    result += MOD as usize;
    result -= 1;
    result %= MOD as usize;

    println!("{}", result);
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
