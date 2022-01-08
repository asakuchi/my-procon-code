// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u128,
    }

    let mut tousa_su_set = std::collections::HashSet::new();

    for first in 1..=9 {
        for digit in -9..=8 {
            let mut text = String::from("");

            let mut number = first;

            for _ in 0..18 {
                text.push_str(&number.to_string());

                tousa_su_set.insert(text.parse::<u128>().unwrap());

                number += digit;
                if number < 0 || number > 9 {
                    break;
                }
            }
        }
    }

    let mut tousa_su_list = tousa_su_set.iter().collect::<Vec<&u128>>();
    tousa_su_list.sort();

    for &number in tousa_su_list {
        if number >= x {
            println!("{}", number);
            break;
        }
    }
}
