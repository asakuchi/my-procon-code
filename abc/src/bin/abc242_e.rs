use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const MODULO: usize = 998244353;

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        let kaibun = format!(
            "{}{}",
            &s[0..(n + 1) / 2]
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(""),
            &s[0..n / 2]
                .iter()
                .rev()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        );

        let mut result = 0;

        let half = &s[0..(n + 1) / 2].iter().map(|c| *c).collect::<Vec<_>>();

        for (i, &c) in half.iter().rev().enumerate() {
            result += power(26, i, MODULO) * (c as u8 - 'A' as u8) as usize;
            result %= MODULO;
        }

        result += 1;
        result %= MODULO;

        if kaibun > s.iter().map(|c| c.to_string()).collect::<Vec<_>>().join("") {
            result += MODULO;
            result -= 1;
            result %= MODULO;
        }

        println!("{}", result);
    }
}

fn power(x: usize, n: usize, modulo: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut result = power(x * x % modulo, n / 2, modulo);

    if n % 2 != 0 {
        result = result * x % modulo;
    }

    result
}
