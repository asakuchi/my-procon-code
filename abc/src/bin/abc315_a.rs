use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ng = vec!['a', 'e', 'i', 'o', 'u'];

    for i in 0..s.len() {
        if !ng.contains(&s[i]) {
            print!("{}", s[i]);
        }
    }

    println!();
}
