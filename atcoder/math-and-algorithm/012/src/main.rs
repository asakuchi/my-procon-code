use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if is_prime(n) {
        println!("Yes");
    } else {
        println!("No");
    }
}

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
