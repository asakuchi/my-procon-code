use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        t: usize,
        test_i: [usize; t],
    }

    let mut result = vec![Vec::new(); t];

    let primes = get_prime(3_001_000);

    for i in 0..t {
        let mut n = test_i[i];

        for &prime in &primes {
            if n % prime == 0 {
                n /= prime;
                result[i].push(prime);

                if n % prime == 0 {
                    n /= prime;
                    result[i].push(prime);
                }

                if result[i].len() == 2 {
                    println!("{} {}", prime, n);
                } else {
                    // n は平方
                    println!("{} {}", n.sqrt(), prime);
                }

                break;
            }
        }
    }
}

///
/// n 以下の素数を返す
/// エラトステネスのふるい
///
/// 1_000_000 10^6 はすぐに帰ってくる
/// 10_000_000 10^7 は少し時間かかる
///
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
