use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let initial = vec!['M', 'A', 'R', 'C', 'H'];

    let mut sets = vec![HashSet::new(); initial.len()];

    for i in 0..n {
        let name = s[i].clone();

        for j in 0..initial.len() {
            if name.starts_with(&initial[j].to_string()) {
                sets[j].insert(name);
                break;
            }
        }
    }

    let mut result = 0;

    for i in 0..initial.len() {
        for j in i + 1..initial.len() {
            for k in j + 1..initial.len() {
                result += sets[i].len() * sets[j].len() * sets[k].len();
            }
        }
    }

    println!("{}", result);
}
