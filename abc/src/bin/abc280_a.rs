use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut result = 0;

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
