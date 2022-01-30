// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut not_a_first_index = 0;
    let mut not_a_last_index = s.len() - 1;

    let mut front_a_count = 0;

    for c in s.iter() {
        if *c == 'a' {
            if not_a_first_index == s.len() - 1 {
                // 全部 a
                println!("Yes");
                return;
            }

            not_a_first_index += 1;
            front_a_count += 1;
        } else {
            break;
        }
    }

    for i in 0..front_a_count {
        if s[s.len() - 1 - i] != 'a' {
            println!("No");
            return;
        }
    }

    for c in s.iter().rev() {
        // println!("{}", c);
        if *c == 'a' {
            if not_a_last_index == 0 {
                // 全部 a
                println!("Yes");
                return;
            }

            not_a_last_index -= 1;
        } else {
            break;
        }
    }

    // println!("not_a_index:{} {}", not_a_first_index, not_a_last_index);

    // if not_a_first_index == not_a_last_index {
    //     println!("Yes");
    //     return;
    // }

    // if not_a_first_index + 1 == not_a_last_index {
    //     if s[not_a_first_index] == s[not_a_last_index] {
    //         println!("Yes");
    //     } else {
    //         println!("No");
    //     }
    //     return;
    // }

    // println!("check");

    let mut result = true;

    for i in 0..s.len() {
        // println!(
        //     "{} {} {}",
        //     i,
        //     s[not_a_first_index + i],
        //     s[not_a_last_index - i]
        // );

        if not_a_first_index + i >= not_a_last_index - i {
            break;
        }

        if s[not_a_first_index + i] != s[not_a_last_index - i] {
            result = false;
            break;
        }
    }

    println!("{}", if result { "Yes" } else { "No" });
}
