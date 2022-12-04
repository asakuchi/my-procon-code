use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let primes = prime_factorize(k);

    let mut k_map = HashMap::new();

    for prime in primes {
        *k_map.entry(prime).or_insert(0) += 1;
    }

    let mut ok: isize = k as isize;
    let mut ng = 1;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let solve = || {
            for (&prime, &count) in k_map.iter() {
                // ルジャンドルの定理
                let mut target_mod_count = 0;

                let mut kata = 1;

                while prime.pow(kata) as isize <= mid {
                    target_mod_count += mid / prime.pow(kata) as isize;

                    kata += 1;
                }

                if target_mod_count < count {
                    return false;
                }
            }

            true
        };

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn prime_factorize(n: usize) -> Vec<usize> {
    let mut current = n;

    let mut list = Vec::new();

    {
        let mut i = 2;

        while i * i <= n {
            while current % i == 0 {
                list.push(i);
                current /= i;
            }

            if current == 1 {
                break;
            }

            i += 1;
        }

        if current != 1 {
            list.push(current);
        }
    }

    list
}
