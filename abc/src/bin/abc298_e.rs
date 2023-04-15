use proconio::input;

use ac_library_rs::ModInt998244353 as mint;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
    }

    let mut result = mint::from(0);

    // n 回 振れば必ずゴールする

    let mut taka = vec![mint::from(0); n + 1];
    let mut aoki = vec![mint::from(0); n + 1];

    taka[a] = mint::from(1);
    aoki[b] = mint::from(1);

    for _ in 0..n {
        let mut next_taka = vec![mint::from(0); n + 1];
        let mut next_aoki = vec![mint::from(0); n + 1];

        for i in 0..=n {
            for j in 1..=p {
                next_taka[(i + j).min(n)] += taka[i] / mint::from(p);
            }
        }

        result += next_taka[n] * (mint::from(1) - aoki[n]);

        next_taka[n] = mint::from(0);

        for i in 0..=n {
            for j in 1..=q {
                next_aoki[(i + j).min(n)] += aoki[i] / mint::from(q);
            }
        }

        taka = next_taka;
        aoki = next_aoki;
    }

    println!("{}", result);
}
