use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }

    let mut r_s = vec![vec!['_'; h]; w];
    let mut r_t = vec![vec!['_'; h]; w];

    for i in 0..h {
        for j in 0..w {
            r_s[j][i] = s[i][j];
            r_t[j][i] = t[i][j];
        }
    }

    r_s.sort();
    r_t.sort();

    if r_s == r_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
