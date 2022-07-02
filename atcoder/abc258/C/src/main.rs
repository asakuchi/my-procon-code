use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: isize,
        q: usize,
        s: Chars,
    }

    let mut header: isize = 0;

    for _ in 0..q {
        input! {
            t : usize,
            x : isize,
        }

        match t {
            2 => {
                let mut point = x + header - 1;

                if point >= n {
                    point -= n;
                }

                println!("{}", s[point as usize]);
            }
            _ => {
                header -= x;

                if header < 0 {
                    header += n;
                }
            }
        }
    }
}
