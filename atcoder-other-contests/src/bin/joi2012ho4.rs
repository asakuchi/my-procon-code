use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b_x: [(Usize1, Usize1, usize); m],
    }

    let mut imos = Vec::new();

    for i in 0..=n + 1 {
        imos.push(vec![0isize; i + 3]);
    }

    for &(a, b, x) in &a_b_x {
        // 6点
        imos[a][b] += 1;
        imos[a][b + 1] -= 1;

        imos[a + x + 1][b] -= 1;
        imos[a + x + 2][b + 1] += 1;

        imos[a + x + 1][b + x + 2] += 1;
        imos[a + x + 2][b + x + 2] -= 1;
    }

    // 左から右
    for i in 0..n {
        for j in 0..=i + 1 {
            if j > 0 {
                imos[i][j] += imos[i][j - 1];
            }
        }
    }

    // 上から左下
    for i in 0..n {
        for j in 0..=i + 1 {
            if i > 0 && j < i + 1 {
                imos[i][j] += imos[i - 1][j];
            }
        }
    }

    // 上から右下
    for i in 0..n {
        for j in 0..=i + 1 {
            if i > 0 && j > 0 {
                imos[i][j] += imos[i - 1][j - 1];
            }
        }
    }

    let mut result = 0usize;

    for i in 0..n {
        for j in 0..=i + 1 {
            if imos[i][j] > 0 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
