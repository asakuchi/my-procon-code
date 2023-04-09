use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut b_index = Vec::new();

    for i in 0..n {
        if s[i] == 'B' {
            b_index.push(i);
        }

        if s[i] == 'K' {
            if !(s[..i].contains(&'R') && s[i..].contains(&'R')) {
                println!("No");
                return;
            }
        }
    }

    if b_index[0] % 2 == b_index[1] % 2 {
        println!("No");
        return;
    }

    println!("Yes");
}
