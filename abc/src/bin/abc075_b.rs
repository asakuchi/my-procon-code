use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                print!("#");
                continue;
            }

            let mut count = 0;

            for diff_i in -1..=1 {
                for diff_j in -1..=1 {
                    let target_i = i as i64 + diff_i;
                    let target_j = j as i64 + diff_j;

                    if target_i < 0 || target_i >= h as i64 || target_j < 0 || target_j >= w as i64
                    {
                        continue;
                    }

                    if s[target_i as usize][target_j as usize] == '#' {
                        count += 1
                    }
                }
            }

            print!("{}", count);
        }

        println!();
    }
}
