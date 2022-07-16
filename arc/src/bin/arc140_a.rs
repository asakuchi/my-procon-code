use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    for chunk in 1..=n {
        if n % chunk != 0 {
            continue;
        }

        let mut count = vec![HashMap::new(); chunk];

        for i in 0..n {
            *count[i % chunk].entry(s[i]).or_insert(0) += 1;
        }

        let mut need_hand = 0;

        for i in 0..chunk {
            let max_count = count[i].iter().map(|(_c, &num)| num).max().unwrap();

            need_hand += (n / chunk) - max_count;
        }

        if need_hand <= k {
            println!("{}", chunk);
            return;
        }
    }
}
