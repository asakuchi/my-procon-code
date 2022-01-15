// -*- coding:utf-8-unix -*-

use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    // println!("start");

    // let mut memo = vec![vec![-2; std::i32::MAX as usize]; std::i32::MAX as usize];
    // let mut memo = vec![vec![-2; 1_000_0]; 1_000_0];

    let mut checked = HashMap::new();

    let mut queue = VecDeque::new();

    let mut times = HashMap::new();

    times.entry(1).or_insert(0);
    queue.push_back(1);

    while let Some(number) = queue.pop_front() {
        // println!("{}", number);

        // let time = { times.entry(number).or_insert(0) };
        let time = times.get(&number).unwrap().clone();

        if number.to_string().len() > n.to_string().len() {
            // println!("len {} {}", number.to_string().len(), n.to_string().len());

            continue;
        }

        if number == n {
            // println!("{} {}", number, n);
            break;
        }

        times.insert(number * a, time + 1);

        if !checked.contains_key(&(number * a)) {
            // *times.entry(number * a).or_insert(0) = time.clone() + 1;
            times.insert(number * a, time + 1);
            checked.insert(number * a, true);

            queue.push_back(number * a);
        }

        let mut rotating = number;

        for _ in 00..number.to_string().len() {
            rotating = rotate(rotating);

            if !checked.contains_key(&rotating) {
                // *times.entry(rotating).or_insert(0) = time.clone() + 1;
                times.insert(rotating, time + 1);
                checked.insert(rotating, true);

                queue.push_back(rotating);
            }
        }
    }

    match times.get(&n) {
        Some(number) => println!("{}", number),
        None => println!("-1"),
    }

    // println!("{}", if yes { "Yes" } else { "No" });
    // println!("{}", yes);
}

// memo: &mut HashMap<(i32,i32),isize>

// fn dfs(
//     a: usize,
//     n: usize,
//     now: usize,
//     memo: &mut HashMap<isize, isize>,
//     memo: &mut HashMap<isize, isize>,
// ) -> isize {
//     println!("{}", now);

//     let text = now.to_string();
//     let n_text = n.to_string();

//     if let Some(&number) = memo.get(&(now as isize)) {
//         println!("memo:{}", number);
//         return number;
//     }

//     if now == n {
//         println!("hit:{}", now);

//         // memo[now][count] = count as i32;
//         return 0;
//     }

//     if text.len() > n_text.len() {
//         return -1;
//     }

//     let multiply_count = dfs(a, n, now * a, memo) + 1;
//     let rotate_count = dfs(a, n, rotate(now), memo) + 1;

//     let result = if multiply_count == -1 && rotate_count == -1 {
//         -1
//     } else if multiply_count == -1 {
//         rotate_count
//     } else if rotate_count == -1 {
//         multiply_count
//     } else {
//         std::cmp::min(rotate_count, multiply_count)
//     };

//     // memo[now][count] = result as i32;
//     *memo.entry(now as isize).or_insert(result) += 1;

//     result
// }

fn rotate(x: usize) -> usize {
    let text = x.to_string();

    let length = text.len();

    let last = &text[length - 1..length];
    let others = &text[0..length - 1];

    let new_text = format!("{}{}", last, others);

    new_text.parse().unwrap()
}
