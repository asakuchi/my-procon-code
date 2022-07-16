use num_integer::*;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut divisors = Vec::new();

    for i in 1..=n.sqrt() {
        if n % i == 0 {
            if n != i {
                divisors.push(i);
            }
            if n / i != n {
                divisors.push(n / i);
            }
        }
    }

    divisors.sort();
    divisors.dedup();

    let sum: usize = divisors.iter().sum();

    if n == sum {
        println!("Perfect");
    } else if n > sum {
        println!("Deficient");
    } else {
        println!("Abundant");
    }
}
