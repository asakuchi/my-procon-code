use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if a[i][j] == 'W' && a[j][i] == 'L' {
                continue;
            }

            if a[i][j] == 'L' && a[j][i] == 'W' {
                continue;
            }

            if a[i][j] == 'D' && a[j][i] == 'D' {
                continue;
            }

            println!("incorrect");
            return;
        }
    }

    println!("correct");
}
