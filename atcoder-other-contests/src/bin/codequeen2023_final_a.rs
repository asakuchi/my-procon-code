use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        c: char,
    }

    let mut list = Vec::new();

    for x in s {
        if x == c {
            list.push(x);
        }
        list.push(x);
    }

    println!("{}", list.iter().collect::<String>());
}
