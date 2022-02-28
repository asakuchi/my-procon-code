fn main() {
    let x = 999993031;

    println!("{} is prime?: {}", x, is_prime(x));
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
