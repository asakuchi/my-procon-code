use std::collections::HashSet;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        s: Chars,
        k: Usize1,
    }

    let mut set = HashSet::new();

    if s.len() > 5 {
        for i in 0..s.len() - 4 {
            rec(&s[i..i + 5], &mut set);
        }
    } else {
        rec(&s, &mut set);
    }

    let mut result: Vec<_> = set.into_iter().collect();
    result.sort();

    println!("{}", result[k]);
}

fn rec(s: &[char], set: &mut HashSet<String>) {
    let text = s.iter().collect();

    if set.contains(&text) {
        return;
    }

    set.insert(text);

    if s.len() == 1 {
        return;
    }

    rec(&s[0..1], set);
    rec(&s[1..], set);
    rec(&s[..s.len() - 1], set);
    rec(&s[s.len() - 1..], set);
}
