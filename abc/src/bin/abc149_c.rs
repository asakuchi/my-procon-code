use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let primes = get_prime(100100);

    for prime in primes {
        if prime >= x {
            println!("{}", prime);
            return;
        }
    }
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
