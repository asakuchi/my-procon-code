use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        x: Chars,
    }

    let keta = x.len();

    let mut s = vec![0; keta + 1];
    s[0] = 0;

    for i in 0..keta {
        let num: usize = x[i].to_string().parse().unwrap();
        s[i + 1] = s[i] + num;
    }

    for i in (1..keta + 1).rev() {
        let now = s[i] % 10;
        let next = s[i] / 10;

        s[i] = now;
        s[i - 1] += next;
    }

    let mut first_zero = true;

    for value in s {
        if value == 0 && first_zero {
            continue;
        } else {
            first_zero = false;
            print!("{}", value);
        }
    }

    println!();
}
