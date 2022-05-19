use itertools::Itertools;
use proconio::input;

const INF: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize,
    }

    if n % 3 == 0 {
        println!("0");
        return;
    }

    let mut list = Vec::new();

    let mut num = n;

    while num != 0 {
        list.push(num % 10);
        num /= 10;
    }

    let mut result = INF;

    'pick_loop: for pick_count in 1..list.len() {
        for combination in (0..list.len()).combinations(pick_count) {
            let mut sum = 0;

            for j in combination {
                sum += list[j];
            }

            if sum % 3 == 0 {
                result = result.min(list.len() - pick_count);
                continue 'pick_loop;
            }
        }
    }

    if result == INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
