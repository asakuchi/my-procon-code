use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [Usize1; n],
    }

    let mut c_list = vec![VecDeque::new(); m];

    for i in 0..n {
        c_list[c[i]].push_back(s[i]);
    }

    for i in 0..m {
        let c = c_list[i].pop_back().unwrap();
        c_list[i].push_front(c);
    }

    let mut result = Vec::new();

    for i in 0..n {
        let c = c_list[c[i]].pop_front().unwrap();
        result.push(c);
    }

    println!("{}", result.into_iter().collect::<String>());
}
