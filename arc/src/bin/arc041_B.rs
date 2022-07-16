use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        b_str: [Chars; n],
    }

    let mut b = vec![vec![0; m]; n];
    let mut a = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            b[i][j] = b_str[i][j].to_digit(10).unwrap();
        }
    }

    for i in 0..n {
        for j in 0..m {
            if b[i][j] != 0 {
                let b_value = b[i][j];

                a[i + 1][j] += b_value;

                b[i][j] -= b_value;
                b[i + 1][j - 1] -= b_value;
                b[i + 1][j + 1] -= b_value;
                b[i + 2][j] -= b_value;
            }
        }
    }

    for i in 0..n {
        let line = a[i]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("");

        println!("{}", line);
    }
}
