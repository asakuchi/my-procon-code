// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u128,
    }

    for num in x..std::u128::MAX {
        let vec_num = num
            .to_string()
            .chars()
            .map(|n| n.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        if is_tousa(vec_num) {
            println!("{}", num);
            break;
        }
    }

    // println!("{}", if yes { "Yes" } else { "No" });
}

fn is_tousa(num: Vec<u32>) -> bool {
    if num.len() < 2 {
        return true;
    }

    let mut answer = true;

    let mut prev = num[0];
    let prev_diff = num[1] as i32 - num[0] as i32;

    for &n in num.iter().skip(1) {
        let diff = n as i32 - prev as i32;

        // println!("n:{} diff:{} prev_diff:{}", n, diff, prev_diff);

        if prev_diff != diff {
            answer = false;
        }

        prev = n;
    }

    answer
}
