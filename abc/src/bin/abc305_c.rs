use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut a = 1_000;
    let mut b = 0;
    let mut c = 1_000;
    let mut d = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                b = i;

                if a == 1_000 {
                    a = i;
                }
            }
        }
    }

    for j in 0..w {
        for i in 0..h {
            if s[i][j] == '#' {
                d = j;

                if c == 1_000 {
                    c = j;
                }
            }
        }
    }

    for i in a..=b {
        for j in c..=d {
            if s[i][j] == '.' {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
