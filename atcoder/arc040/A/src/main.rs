use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut taka = 0;
    let mut aoki = 0;

    for i in 0..n {
        for j in 0..n {
            match s[i][j] {
                'R' => taka += 1,
                'B' => aoki += 1,
                _ => {}
            }
        }
    }

    if taka > aoki {
        println!("TAKAHASHI");
    } else if aoki > taka {
        println!("AOKI");
    } else {
        println!("DRAW");
    }
}
