use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        s: [Chars; h],
    }

    let mut result = 1; // (X,Y)の分

    for i in x + 1..h {
        if s[i][y] == '#' {
            break;
        }

        result += 1;
    }

    for i in (0..x).rev() {
        if s[i][y] == '#' {
            break;
        }

        result += 1;
    }

    for j in y + 1..w {
        if s[x][j] == '#' {
            break;
        }

        result += 1;
    }

    for j in (0..y).rev() {
        if s[x][j] == '#' {
            break;
        }

        result += 1;
    }

    println!("{}", result);
}
