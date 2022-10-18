use std::collections::HashMap;

use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n_input: usize,
    }

    // let mut list = Vec::new();
    let mut map = HashMap::new();

    for n in 1..=n_input {
        let mut i = 2;
        let mut current = n;

        while i * i <= n {
            while current % i == 0 {
                // list.push(i);
                *map.entry(i).or_insert(0) += 1;
                current /= i;
            }

            if current == 1 {
                break;
            }

            i += 1;
        }

        if current != 1 {
            // list.push(current);
            *map.entry(current).or_insert(0) += 1;
        }
    }

    let mut result = 1;

    for (&_key, &value) in map.iter() {
        result *= value + 1;
        result %= MOD;
    }

    println!("{}", result % MOD);
}
