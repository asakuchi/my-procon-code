use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[isize; w]; h],
        mut b: [[isize; w]; h],
    }

    let mut result = 0;

    for i in 0..h - 1 {
        for j in 0..w - 1 {
            if a[i][j] != b[i][j] {
                let diff = b[i][j] - a[i][j];

                a[i][j] += diff;
                a[i + 1][j] += diff;
                a[i][j + 1] += diff;
                a[i + 1][j + 1] += diff;

                result += diff.abs();
            }
        }

        if a[i][w - 1] != b[i][w - 1] {
            println!("No");
            return;
        }
    }

    for j in 0..w {
        if a[h - 1][j] != b[h - 1][j] {
            println!("No");
            return;
        }
    }

    println!("Yes");
    println!("{}", result);
}
