use itertools::Itertools;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        a: [usize; n],
    }

    let first = a[..n / 2].to_vec();
    let latter = a[n / 2..].to_vec();

    let mut first_counter = vec![Vec::new(); n + 1];
    let mut latter_counter = vec![Vec::new(); n + 1];

    first_counter[0].push(0);
    latter_counter[0].push(0);

    for size in 1..=k {
        if size > first.len() {
            break;
        }

        for combination in first.iter().combinations(size) {
            let mut price = 0;

            for combi in combination {
                price += combi;
            }

            if price <= p {
                first_counter[size].push(price);
            }
        }
    }

    for size in 1..=k {
        if size > latter.len() {
            break;
        }

        for combination in latter.iter().combinations(size) {
            let mut price = 0;

            for combi in combination {
                price += combi;
            }

            if price <= p {
                latter_counter[size].push(price);
            }
        }
    }

    for size in 1..k {
        first_counter[size].sort();
        latter_counter[size].sort();
    }

    let mut result = 0;

    for size in 0..n {
        for &value in &first_counter[size] {
            let target = latter_counter[k - size].upper_bound(&(p - value));

            result += target;
        }
    }

    println!("{}", result);
}
