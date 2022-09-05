use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut is_prime = vec![true; n + 1];

    is_prime[0] = false;
    is_prime[1] = false;

    let mut prime_count = vec![0; n + 1];

    for i in 2..=n {
        if is_prime[i] {
            // 素数自身
            prime_count[i] = 1;

            for j in (i * 2..=n).step_by(i) {
                is_prime[j] = false;
                prime_count[j] += 1;
            }
        }
    }

    let mut result = 0;

    for i in 2..=n {
        if prime_count[i] >= k {
            result += 1;
        }
    }

    println!("{}", result);
}
