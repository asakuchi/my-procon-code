use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }

            let mut ok = false;

            for &direction in &directions {
                let x = i as isize + direction.0;
                let y = j as isize + direction.1;

                if x < 0 || x >= h as isize || y < 0 || y >= w as isize {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if s[x][y] == '#' {
                    ok = true;
                }
            }

            if !ok {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
