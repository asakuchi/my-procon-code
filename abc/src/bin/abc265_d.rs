use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n],
    }

    let mut acc = HashSet::new();
    let mut s = vec![0; n + 1];

    for i in 0..n {
        s[i + 1] = s[i] + a[i];
        acc.insert(s[i + 1]);
    }

    for i in 0..n {
        if acc.contains(&(s[i] + p))
            && acc.contains(&(s[i] + p + q))
            && acc.contains(&(s[i] + p + q + r))
        {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
