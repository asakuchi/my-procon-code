// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    let mut queue = VecDeque::new();

    let mut times = vec![-1; 1_000_000];

    queue.push_back(1);
    times[1] = 0;

    while let Some(number) = queue.pop_front() {
        if number == n {
            // println!("{} {}", number, n);
            break;
        }

        if number.to_string().len() > n.to_string().len() {
            continue;
        }

        let time = times[number];

        let multiplied_number = number * a;

        if multiplied_number.to_string().len() <= n.to_string().len()
            && times[multiplied_number] == -1
        {
            queue.push_back(multiplied_number);
            times[multiplied_number] = time + 1;
        }

        if number >= 10 && number % 10 != 0 {
            let rotated_number = rotate(number);

            if times[rotated_number] == -1 {
                queue.push_back(rotated_number);
                times[rotated_number] = time + 1;
            }
        }
    }

    println!("{}", times[n]);
}

fn rotate(x: usize) -> usize {
    let text = x.to_string();

    let length = text.len();

    let last = &text[length - 1..length];
    let others = &text[0..length - 1];

    let new_text = format!("{}{}", last, others);

    new_text.parse().unwrap()
}
