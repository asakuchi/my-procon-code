use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    if s.len() <= 2 {
        if s[0] == s[1] {
            println!("1 2");
        } else {
            println!("-1 -1");
        }

        return;
    }

    let mut map = std::collections::HashMap::new();

    *map.entry(s[0]).or_insert(0) += 1;
    let count = map.entry(s[1]).or_insert(0);
    *count += 1;
    if *count >= 2 {
        println!("{} {}", 1, 3);
        return;
    }

    for i in 2..s.len() {
        let count = map.entry(s[i]).or_insert(0);
        *count += 1;

        if *count >= 2 {
            println!("{} {}", i - 1, i + 1);
            return;
        }

        let count = map.entry(s[i - 2]).or_insert(0);
        *count -= 1;
    }

    println!("-1 -1");
}
