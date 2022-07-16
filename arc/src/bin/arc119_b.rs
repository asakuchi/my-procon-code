use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }

    let s_0: Vec<_> = s
        .iter()
        .enumerate()
        .filter(|(_i, &c)| c == '0')
        .map(|(i, &_c)| i)
        .collect();
    let t_0: Vec<_> = t
        .iter()
        .enumerate()
        .filter(|(_i, &c)| c == '0')
        .map(|(i, &_c)| i)
        .collect();

    if s_0.len() != t_0.len() {
        println!("{}", -1);
        return;
    }

    let mut result = 0;

    for i in 0..s_0.len() {
        if s_0[i] != t_0[i] {
            result += 1;
        }
    }

    println!("{}", result);
}
