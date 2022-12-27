const MAX_NUM: usize = 1_000_000;

fn main() {
    let x = 999993031;

    println!("{} is prime?: {}", x, is_prime(x));

    let primes = get_prime(MAX_NUM);
    println!("{}", primes.len());

    // is_prime で作ったリストと get_prime の比較
    let mut primes_2 = Vec::with_capacity(MAX_NUM);

    for i in 2..=MAX_NUM {
        if is_prime(i) {
            primes_2.push(i);
        }
    }

    assert_eq!(primes, primes_2);

    let factors = prime_factorize(207_900);

    println!("{:?}", factors);

    let min_factor = pre_osa_k(MAX_NUM);
    let primes_osa_k = osa_k(207_900, &min_factor);

    // println!("{:?}", &min_factor[2..20]);
    println!("{}", factors == primes_osa_k);
}

///
/// 素数判定
///
fn is_prime(x: usize) -> bool {
    if x == 2 {
        return true;
    }

    if x < 2 || x % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i = i + 2;
    }

    true
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

///
/// 高速な素因数分解（前処理）
///
fn pre_osa_k(n: usize) -> Vec<usize> {
    let mut min_factor: Vec<_> = (0..=n).collect();

    let mut i = 2;

    while i * i <= n {
        if min_factor[i] == i {
            for k in (i * 2..=n).step_by(i) {
                if min_factor[k] > i {
                    min_factor[k] = i;
                }
            }
        }

        i += 1;
    }

    min_factor
}

///
/// 高速な素因数分解
///
fn osa_k(m: usize, min_factor: &Vec<usize>) -> Vec<usize> {
    let mut k = m;

    let mut primes = Vec::new();

    while k >= 2 {
        primes.push(min_factor[k]);

        k /= min_factor[k];
    }

    primes
}

///
/// 約数列挙
///
fn divisors(n: usize) -> Vec<usize> {
    let mut list = Vec::new();

    {
        let mut i = 1;

        while i * i <= n {
            if n % i == 0 {
                list.push(i);

                if n / i != i {
                    list.push(n / i);
                }
            }

            i += 1;
        }
    }

    list.sort();

    list
}
