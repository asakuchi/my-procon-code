use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut list = vec![vec!['.'; n * b]; n * a];

    for i in 0..n {
        for j in 0..n {
            if (i + j) % 2 == 0 {
                continue;
            }

            for k in 0..a {
                for l in 0..b {
                    list[a * i + k][b * j + l] = '#';
                }
            }
        }
    }

    for i in 0..n * a {
        for j in 0..n * b {
            print!("{}", list[i][j]);
        }

        println!();
    }
}
