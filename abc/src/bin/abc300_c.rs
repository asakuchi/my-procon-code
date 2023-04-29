use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let n = h.min(w);

    let mut result = vec![0; n];

    for i in 0..h {
        for j in 0..w {
            if c[i][j] != '#' {
                continue;
            }

            let mut ok_size = 0;

            for size in 1..=n {
                let mut ok = true;

                for direction in vec![(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    let next_i = i as isize + direction.0 * size as isize;
                    let next_j = j as isize + direction.1 * size as isize;

                    if next_i < 0 || next_i >= h as isize || next_j < 0 || next_j >= w as isize {
                        ok = false;
                        break;
                    }

                    let next_i = next_i as usize;
                    let next_j = next_j as usize;

                    if c[next_i][next_j] != '#' {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    ok_size = size;
                } else {
                    break;
                }
            }

            if ok_size > 0 {
                result[ok_size - 1] += 1;
            }
        }
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}
