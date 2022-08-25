use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
    }

    if n == 0 {
        println!("0");
        return;
    }

    let mut x = n;

    let mut counter = HashMap::new();

    let mut loop_list = Vec::new();

    loop {
        let count = counter.entry(x).or_insert(0);
        *count += 1;

        if *count == 2 {
            loop_list.push(x);
        } else if *count == 3 {
            break;
        }

        let y: usize = x
            .to_string()
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .sum();

        x = (x + y) % 100_000;
        k -= 1;

        if k == 0 {
            println!("{}", x);
            return;
        }
    }

    k %= loop_list.len();

    print!("{}", loop_list[k]);
}
