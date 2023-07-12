// 解説AC

use proconio::{input, marker::Chars};

use ac_library_rs::ModInt1000000007 as mint;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut x = vec![vec![mint::from(0); w + 1]; h + 1];
    let mut y = vec![vec![mint::from(0); w + 1]; h + 1];
    let mut z = vec![vec![mint::from(0); w + 1]; h + 1];

    let mut count = vec![vec![mint::from(0); w + 1]; h + 1];

    count[1][1] = mint::from(1);

    // let patterns = vec![(1, 0), (0, 1), (1, 1)];

    for i in 1..=h {
        for j in 1..=w {
            if s[i - 1][j - 1] == '#' {
                continue;
            }

            count[i][j] += x[i][j - 1] + y[i - 1][j] + z[i - 1][j - 1];

            let prev_x = x[i][j - 1];
            x[i][j] += prev_x + count[i][j];
            let prev_y = y[i - 1][j];
            y[i][j] += prev_y + count[i][j];
            let prev_z = z[i - 1][j - 1];
            z[i][j] += prev_z + count[i][j];
        }
    }

    println!("{}", count[h][w]);
}
