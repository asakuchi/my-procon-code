use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 10],
    }

    let mut a = None;
    let mut b = 0;
    let mut c = None;
    let mut d = 0;

    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                if a == None {
                    a = Some(i);
                }

                if c == None {
                    c = Some(j);
                }

                b = i;
                d = j;
            }
        }
    }

    println!("{} {}", a.unwrap() + 1, b + 1);
    println!("{} {}", c.unwrap() + 1, d + 1);
}
