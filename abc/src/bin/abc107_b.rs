use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut skip_row = HashSet::new();
    let mut skip_column = HashSet::new();

    for i in 0..h {
        let mut has_black = false;

        for j in 0..w {
            if a[i][j] == '#' {
                has_black = true;
                break;
            }
        }

        if !has_black {
            skip_row.insert(i);
        }
    }

    for j in 0..w {
        let mut has_black = false;

        for i in 0..h {
            if a[i][j] == '#' {
                has_black = true;
                break;
            }
        }

        if !has_black {
            skip_column.insert(j);
        }
    }

    for i in 0..h {
        if skip_row.contains(&i) {
            continue;
        }

        let mut line = Vec::new();

        for j in 0..w {
            if skip_column.contains(&j) {
                continue;
            }

            line.push(a[i][j]);
        }

        println!("{}", line.iter().collect::<String>());
    }
}
