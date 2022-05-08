use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    let primes = get_prime(1_000_000);

    let mut result = 0;

    for i in 0..primes.len() {
        for j in i + 1..primes.len() {
            let k = primes[i] as u128 * (primes[j] as u128).pow(3);

            if k <= n {
                result += 1;
            } else {
                break;
            }
        }
    }

    println!("{}", result);
}

fn get_prime(n: usize) -> Vec<usize> {
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

    list
}
