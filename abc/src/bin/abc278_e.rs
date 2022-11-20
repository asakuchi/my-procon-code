use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        diff_h: usize,
        diff_w: usize,
        a: [[usize; w]; h],
    }

    let mut s = vec![vec![vec![0; n + 1]; w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            for x in 1..=n {
                s[i + 1][j + 1][x] = s[i][j + 1][x] + s[i + 1][j][x] - s[i][j][x];
            }

            let x = a[i][j];

            s[i + 1][j + 1][x] += 1;
        }
    }

    let mut kind = 0;

    for x in 1..=n {
        if s[h][w][x] != 0 {
            kind += 1;
        }
    }

    // println!("kind {}", kind);
    // println!("{:?}", s);

    for i in 0..h - diff_h + 1 {
        for j in 0..w - diff_w + 1 {
            // println!("output {} {}", i, j);

            let l_x = i;
            let r_x = i + diff_h;
            let l_y = j;
            let r_y = j + diff_w;

            let mut result_kind = kind;

            for x in 1..=n {
                let x_count = s[h][w][x];

                let minus_x_count =
                    s[r_x][r_y][x] - s[r_x][l_y][x] - s[l_x][r_y][x] + s[l_x][l_y][x];

                // println!("x {} x_count {} minux {}", x, x_count, minus_x_count);

                if x_count != 0 && x_count == minus_x_count {
                    result_kind -= 1;
                }
            }

            println!("{}", result_kind);
        }
    }
}
