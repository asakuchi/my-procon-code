use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut omote_count = 0;

    for &c in &s {
        if c == '1' {
            omote_count += 1;
        }
    }

    if omote_count == 0 {
        println!("0");
        return;
    }

    if omote_count % 2 != 0 {
        println!("-1");
        return;
    }

    if omote_count >= 4 {
        println!("{}", omote_count / 2);
        return;
    }

    // 以下、表の数は2個

    let list = run_length_encoding(&s);

    // 1成分の数
    let mut one_count = 0;

    for item in list {
        if item.0 == '1' {
            one_count += 1;
        }
    }

    if one_count == 2 {
        println!("1");
        return;
    }

    // 以下、表の数が2個で隣接している

    if n >= 5 {
        println!("2");
        return;
    }

    let text = s.iter().format("").to_string();

    if n == 4 {
        // 3 通り
        // 1100
        // 0110
        // 0011

        if text == "0110" {
            println!("3");
            return;
        } else {
            println!("2");
            return;
        }
    }

    // n == 3
    // 2通り
    // 110
    // 011
    // NG
    println!("-1");
}

fn run_length_encoding<T>(list: &Vec<T>) -> Vec<(T, usize)>
where
    T: PartialEq + Copy,
{
    let mut result = Vec::new();

    for &value in list.iter() {
        if let Some((next, size)) = result.pop() {
            if next == value {
                result.push((next, size + 1));
            } else {
                result.push((next, size));
                result.push((value, 1));
            }
        } else {
            result.push((value, 1));
        }
    }

    result
}
