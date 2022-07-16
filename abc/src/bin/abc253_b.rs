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

    let mut list = Vec::new();

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                list.push((i as isize, j as isize));
            }
        }
    }

    let result = (list[0].0 - list[1].0).abs() + (list[0].1 - list[1].1).abs();

    println!("{}", result);
}
