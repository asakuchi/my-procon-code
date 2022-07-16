use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let primes = get_prime(200);
    // println!("{:?}", primes);
    let mut taka_win = false;

    for i in a..=b {
        let mut aoki_win = false;
        // println!("taka:{}", i);

        for j in c..=d {
            // println!("aoki:{}", j);
            if primes.contains(&(i + j)) {
                // println!("is prime win aoki");

                aoki_win = true;
                break;
            }
        }

        if !aoki_win {
            // println!("cannot prime taka win");
            taka_win = true;
            break;
        }
    }

    if taka_win {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

pub fn get_prime(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut list = Vec::with_capacity(n);

    for i in 2..=n {
        if is_prime[i] {
            list.push(i);

            for j in (i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    list
}
