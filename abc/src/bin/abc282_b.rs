use proconio::input;

use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut result = 0;

    for i in 0..n {
        for j in i + 1..n {
            let mut ok = true;

            for k in 0..m {
                if s[i][k] == 'x' && s[j][k] == 'x' {
                    ok = false;
                }
            }

            if ok {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
