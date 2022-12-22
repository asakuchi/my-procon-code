use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = vec![0; n];

    // i(n/2<i)の倍数が書かれた箱は1個のみ
    for i in n / 2 + 1..=n {
        b[i - 1] = a[i - 1];
    }

    for i in (1..=n / 2).rev() {
        let mut total = 0;

        for j in (i * 2..=n).step_by(i) {
            total += b[j - 1];
        }

        if a[i - 1] != total % 2 {
            b[i - 1] = 1;
        }
    }

    let mut result = Vec::new();

    for i in 0..n {
        if b[i] == 1 {
            result.push(i + 1);
        }
    }

    println!("{}", result.len());

    let text = result.iter().join(" ");

    println!("{}", text);
}
