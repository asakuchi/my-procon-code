use proconio::fastout;
use proconio::input;

const MAX_NUM: usize = 100_001;

#[fastout]
fn main() {
    input! {
        q: usize,
        mut lr: [(usize, usize); q],
    }

    let primes = get_prime(MAX_NUM);
    let mut like_2017 = vec![false; MAX_NUM];

    for i in (1..MAX_NUM).step_by(2) {
        if primes[i] && primes[(i + 1) / 2] {
            like_2017[i] = true;
        }
    }

    let mut s = vec![0; MAX_NUM + 1];
    s[0] = 0;

    for i in 0..MAX_NUM {
        s[i + 1] = s[i];

        if like_2017[i] {
            s[i + 1] += 1;
        }
    }

    for (l, r) in lr {
        println!("{}", s[r + 1] - s[l]);
    }
}

pub fn get_prime(n: usize) -> Vec<bool> {
    assert!(n >= 2, "n must be 2 or more");

    let mut is_prime = vec![true; n + 1];
    let mut list = Vec::with_capacity(n);

    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            list.push(i);

            for j in (i * 2..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
}
