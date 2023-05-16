use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    let mut count = HashMap::new();

    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1_usize;
    }

    for i in 0..n {
        let x = a[i];

        let divisors = divisors(x);

        for divisor in divisors {
            if let Some(&count_div) = count.get(&divisor) {
                let other = x / divisor;

                if let Some(&count_other) = count.get(&other) {
                    result += count_div * count_other;
                }
            }
        }
    }

    println!("{}", result);
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
