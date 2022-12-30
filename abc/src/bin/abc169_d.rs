use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let primes = prime_factorize(n);

    let mut map = HashMap::new();

    for prime in primes {
        *map.entry(prime).or_insert(0) += 1;
    }

    let mut result = 0;

    for (_prime, count) in map {
        let mut rest_count = count;
        let mut i = 1;

        while rest_count >= i {
            result += 1;
            rest_count -= i;

            i += 1;
        }
    }

    println!("{}", result);
}

///
/// 素因数分解
///
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
