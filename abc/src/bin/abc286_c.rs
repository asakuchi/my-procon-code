use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }

    let mut list = VecDeque::from(s);

    let mut result = 5_000_000_000_000_000_000;

    let diff = kaibun(&list, n);

    result = result.min(diff * b);

    for i in 1..=n {
        let c = list.pop_front().unwrap();
        list.push_back(c);

        let diff = kaibun(&list, n);

        result = result.min(diff * b + a * i);
    }

    println!("{}", result);
}

fn kaibun(list: &VecDeque<char>, n: usize) -> usize {
    let mut result = 0;

    for i in 0..n / 2 {
        if list[i] != list[n - 1 - i] {
            result += 1;
        }
    }

    result
}
