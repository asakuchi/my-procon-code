use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }

    let mut result = 0;

    for mask in 0..1 << (h + w) {
        let mut black = 0;

        for i in 0..h {
            for j in 0..w {
                if mask & 1 << i > 0 && mask & 1 << (j + h) > 0 && c[i][j] == '#' {
                    black += 1;
                }
            }
        }

        if black == k {
            result += 1;
        }
    }

    println!("{}", result);
}
