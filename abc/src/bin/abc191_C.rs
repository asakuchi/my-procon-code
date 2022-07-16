use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut result = 0;

    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let mut count = 0;

            for k in 0..2 {
                for l in 0..2 {
                    if s[i + k][j + l] == '#' {
                        count += 1;
                    }
                }
            }

            if count == 1 || count == 3 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
