use num_rational::Ratio;
use proconio::input;

const MOD: isize = 998244353;

// WA

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![Ratio::new(0, m as isize); n + 1];

    dp[0] = Ratio::new(1, 1);

    let dice = Ratio::new(1, m as isize);

    // println!("init {:?}", dp);

    let mut result_p = Ratio::new(0, m as isize);

    for _l_count in 0..k {
        let mut p = vec![Ratio::new(0, m as isize); n + 1];
        std::mem::swap(&mut dp, &mut p);
        // p[0] = Ratio::new(1, 1);

        for i in 0..n {
            let source = p[i].clone();

            // println!("i: {} dp[i]: {}", i, p[i]);

            for j in 1..=m {
                let target = if i + j <= n { i + j } else { 2 * n - i - j };

                dp[target] += source * dice;
            }
        }

        // println!("loop {} {:?}", l_count, dp);

        result_p += dp[n];
    }

    // println!("{:?}", result_p);

    let result =
        *result_p.numer() as isize % MOD * mod_inverse(*result_p.denom() as isize, MOD) % MOD;

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
