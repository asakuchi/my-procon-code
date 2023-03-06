use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut rev_n = n.clone();
    rev_n.reverse();

    if n == rev_n {
        println!("Yes");
    } else {
        println!("No");
    }
}
