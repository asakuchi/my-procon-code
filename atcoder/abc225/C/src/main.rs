use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n],
    }

    for i in 0..n {
        for j in 0..m {
            if b[i][j] % 7 == 1 && j != 0 {
                println!("No");
                return;
            }

            if b[i][j] % 7 == 0 && j != m - 1 {
                println!("No");
                return;
            }

            if i > 0 && b[i][j] - b[i - 1][j] != 7 {
                println!("No");
                return;
            }

            if j > 0 && b[i][j] - b[i][j - 1] != 1 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
