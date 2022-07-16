// -*- coding:utf-8-unix -*-

// use proconio::input;

fn main() {
    // if, else
    println!("{}", is_even(10));
    println!("{}", is_even(11));

    // for
    println!("{}", get_range_sum(10));

    // 配列
    println!("{}", get_max_num(vec![1, 20, 3]));

    // ソート
    let mut numbers = vec![3, 4, 7, 2, 5, 6, 1];
    numbers.sort();
    println!("{:?}", numbers);

    // 文字列処理
    let s = "abc";

    // 同値判定
    if s == "abc" {
        println!("equal");
    }

    // 1文字の切り出し
    let c = s.chars().nth(0).unwrap();
    println!("{}", c);

    // 文字の連結
    let s = format!("{}{}{}", "def", s, "ghi");
    println!("{}", s);

    // 文字の切り出し
    let s = &s[3..6];
    println!("{}", s);

    // 連想配列
    count_strings(vec!["hello", "world", "hello"]);
}

///
/// if,else
/// Rust ではif文ではなくif式
///
fn is_even(a: i32) -> i32 {
    if a % 2 == 0 {
        1
    } else {
        0
    }
}

///
/// for
///
fn get_range_sum(n: i32) -> i32 {
    let mut sum = 0;

    // 1..n で n を含まない
    // 1..=n で n を含む
    for i in 1..=n {
        sum += i;
    }

    sum
}

///
/// 配列
/// （↓はVecだが・・・）
///
fn get_max_num(array: Vec<i32>) -> i32 {
    let mut ret = array[0];

    for value in array {
        if ret < value {
            ret = value;
        }
    }

    ret
}

///
/// 連想配列
///
fn count_strings(s: Vec<&str>) {
    // 連想配列

    let mut hm = std::collections::HashMap::new();

    for text in s {
        *hm.entry(text).or_insert(0) += 1;
    }

    for (key, value) in hm {
        println!("{} {}", key, value);
    }
}
