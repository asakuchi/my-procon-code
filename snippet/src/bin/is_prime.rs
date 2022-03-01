const MAX_NUM: usize = 1_000_000;

fn main() {
    let x = 999993031;

    println!("{} is prime?: {}", x, is_prime(x));

    let primes = get_prime(MAX_NUM);
    println!("{}", primes.len());
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

    while i as f64 <= (x as f64).sqrt() {
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
pub fn get_prime(n: usize) -> Vec<usize> {
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