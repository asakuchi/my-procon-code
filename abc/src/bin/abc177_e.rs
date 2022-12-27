use std::collections::HashMap;

use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    let min_factor = pre_osa_k(1_000_000);

    for i in 0..n {
        let mut primes = osa_k(a[i], &min_factor);

        primes.dedup();

        for prime in primes {
            *map.entry(prime).or_insert(0) += 1;
        }
    }

    let mut is_pairwise = true;

    for (_prime, count) in map.into_iter() {
        if count >= 2 {
            is_pairwise = false;
            break;
        }
    }

    if is_pairwise {
        println!("pairwise coprime");
        return;
    }

    let mut gcd = a[0];

    for i in 0..n {
        gcd = gcd.gcd(&a[i]);
    }

    if gcd == 1 {
        println!("setwise coprime");
        return;
    }

    println!("not coprime");

    // return;
}

///
/// 高速な素因数分解（前処理）
///
fn pre_osa_k(n: usize) -> Vec<usize> {
    let mut min_factor: Vec<_> = (0..=n).collect();

    let mut i = 2;

    while i * i <= n {
        if min_factor[i] == i {
            for k in (i * 2..=n).step_by(i) {
                if min_factor[k] > i {
                    min_factor[k] = i;
                }
            }
        }

        i += 1;
    }

    min_factor
}

///
/// 高速な素因数分解
///
fn osa_k(m: usize, min_factor: &Vec<usize>) -> Vec<usize> {
    let mut k = m;

    let mut primes = Vec::new();

    while k >= 2 {
        primes.push(min_factor[k]);

        k /= min_factor[k];
    }

    primes
}
