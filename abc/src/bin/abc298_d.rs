use std::collections::VecDeque;

use proconio::{fastout, input};

const MOD: usize = 998244353;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut list = VecDeque::new();
    list.push_back(1);

    let mut result = 1;

    for _ in 0..q {
        input! {
            c: usize,
        }

        if c == 1 {
            input! {
                x: usize,
            }

            list.push_back(x);

            result = result * 10 + x;
            result %= MOD;
        } else if c == 2 {
            let keta = list.len();
            let head = list.pop_front().unwrap();

            result = result + MOD - (head * mod_pow(10, keta - 1) % MOD);
            result %= MOD;
        } else {
            println!("{}", result % MOD);
        }
    }
}

fn mod_pow(x: usize, a: usize) -> usize {
    if a == 1 {
        return x % MOD;
    }

    if a % 2 == 1 {
        return (x * mod_pow(x, a - 1)) % MOD;
    }

    let t = mod_pow(x, a / 2);

    return (t * t) % MOD;
}
