// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let mut result = true;

    let diff = (t.chars().nth(0).unwrap() as i32 - s.chars().nth(0).unwrap() as i32 + 26) % 26;

    for (c1, c2) in s.chars().zip(t.chars()) {
        let num1 = c1 as i32;
        let num2 = c2 as i32;

        if diff != (num2 - num1 + 26) % 26 {
            result = false;
            break;
        }
    }

    println!("{}", if result { "Yes" } else { "No" });
}
