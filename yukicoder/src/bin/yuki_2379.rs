use std::collections::HashSet;

fn main() {
    let n: usize = input_value();

    let primes = prime_factorize(n);

    let mut set = HashSet::new();

    for prime in primes {
        set.insert(prime);
    }

    if set.len() <= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn input_value<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n = buf.parse().unwrap();

    n
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
