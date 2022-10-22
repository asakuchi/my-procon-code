use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut list = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                list[j] += 1;
            }
        }
    }

    let result = list.iter().format(" ");
    println!("{}", result);
}
