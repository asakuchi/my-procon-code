use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut list = Vec::new();

    let mut mode = 0;

    let mut a_count = 0;
    let mut c_count = 0;

    for c in s {
        match mode {
            1 => {
                // a
                if c == 'A' {
                    a_count += 1;
                } else if c == 'R' {
                    mode = 2;
                } else {
                    mode = 0;
                    a_count = 0;
                    c_count = 0;
                }
            }
            2 => {
                // c
                if c == 'C' {
                    c_count += 1;
                } else {
                    if c_count > 0 {
                        list.push(a_count.min(c_count));
                    }

                    mode = 0;
                    a_count = 0;
                    c_count = 0;

                    if c == 'A' {
                        a_count = 1;
                        mode = 1;
                    }
                }
            }
            _ => {
                if c == 'A' {
                    a_count = 1;
                    mode = 1;
                }
            }
        }
    }

    if mode == 2 {
        if c_count > 0 {
            list.push(a_count.min(c_count));
        }
    }

    let sum: usize = list.iter().sum();

    println!("{}", sum.min(2 * list.len()));
}
