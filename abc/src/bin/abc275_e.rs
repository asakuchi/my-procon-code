use proconio::input;

const MOD: isize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![0; n + 1];

    dp[0] = 1;

    let dice = mod_inverse(m as isize, MOD) % MOD;

    // println!("init {:?}", dp);

    let mut result_p = 0;

    for _l_count in 0..k {
        let mut p = vec![0; n + 1];
        std::mem::swap(&mut dp, &mut p);

        for i in 0..n {
            let source = p[i].clone();

            // println!("i: {} dp[i]: {}", i, p[i]);

            for j in 1..=m {
                let target = if i + j <= n { i + j } else { 2 * n - i - j };

                dp[target] += source * dice;
                dp[target] %= MOD;
            }
        }

        // println!("loop {} {:?}", l_count, dp);

        result_p += dp[n];
        result_p %= MOD;
    }

    // println!("{:?}", result_p);

    let result = result_p;

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
