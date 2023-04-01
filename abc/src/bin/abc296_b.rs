use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    }

    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                println!("{}{}", ('a' as u8 + j as u8) as char, (8 - i));

                return;
            }
        }
    }
}
