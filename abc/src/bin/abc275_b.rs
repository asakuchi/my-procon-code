use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    }

    let mut abc = (a % MOD) * (b % MOD) % MOD * (c % MOD);
    abc %= MOD;

    let mut def = (d % MOD) * (e % MOD) % MOD * (f % MOD);
    def %= MOD;

    let mut result = abc + MOD - def;
    result %= MOD;

    println!("{}", result);
}
