use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut result = true;

    let mut point = -1;
    let mut second_or_third = false;

    for c in s {
        if point == -1 {
            if c == 'o' {
                point = 1;
            } else {
                point = 2;
                second_or_third = true;
            }
        } else {
            if point == 1 && c == 'x' {
                point = 2;
            } else if point == 2 && c == 'x' {
                point = 3;
            } else if (point == 3 && c == 'o') || second_or_third {
                point = 1;
            } else {
                result = false;
                break;
            }

            second_or_third = false;
        }
    }

    println!("{}", if result { "Yes" } else { "No" });
}
