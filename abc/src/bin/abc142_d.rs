use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let min_num = a.min(b);
    let max_num = a.max(b);

    let mut primes = prime_factorize(min_num);

    primes.push(1);
    primes.sort();
    primes.dedup();

    let mut result = 0;

    for prime in primes {
        if max_num % prime == 0 {
            result += 1;
        }
    }

    println!("{}", result);
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
