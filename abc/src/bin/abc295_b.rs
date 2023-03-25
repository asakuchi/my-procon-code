use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r],
    }

    let mut result = vec![vec!['#'; c]; r];

    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '#' {
                // do nothin.
            } else if b[i][j] == '.' {
                result[i][j] = '.';
            } else {
                let x: isize = b[i][j].to_string().parse().unwrap();

                for i_2 in 0..r {
                    for j_2 in 0..c {
                        if (i as isize - i_2 as isize).abs() + (j as isize - j_2 as isize).abs()
                            <= x
                        {
                            result[i_2][j_2] = '.';
                        }
                    }
                }
            }
        }
    }

    for i in 0..r {
        let text = result[i].iter().format("");

        println!("{}", text);
    }
}
