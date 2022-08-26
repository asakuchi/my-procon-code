use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        mut n: usize,
        x: usize,
        m: usize,
    }

    let mut a = x;

    let mut counter = HashMap::new();
    let mut loop_list = Vec::new();

    let mut sum = 0;
    let mut loop_sum = 0;

    loop {
        let count = counter.entry(a).or_insert(0);

        *count += 1;

        if *count == 2 {
            loop_list.push(a);
            loop_sum += a;
        } else if *count == 3 {
            break;
        }

        sum += a;
        n -= 1;

        if n == 0 {
            println!("{}", sum);
            return;
        }

        a = (a * a) % m;
    }

    sum += (n / loop_list.len()) * loop_sum;

    n %= loop_list.len();

    for i in 0..n {
        sum += loop_list[i];
    }

    println!("{}", sum);
}
