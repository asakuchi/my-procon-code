use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
    }

    let mut first = s[0..n].to_vec();
    let mut second = s[n..2 * n].to_vec();

    let mut reversed = false;

    for _ in 0..q {
        input! {
            t: usize,
            a: usize,
            b: usize,
        }

        match t {
            1 => {
                let a = a - 1;
                let b = b - 1;

                let (a, b) = if reversed {
                    ((a + n) % (2 * n), (b + n) % (2 * n))
                } else {
                    (a, b)
                };

                if b < n {
                    if a < n {
                        let tmp = first[a];
                        first[a] = first[b];
                        first[b] = tmp;
                    } else {
                        let a = a - n;

                        let tmp = second[a];
                        second[a] = first[b];
                        first[b] = tmp;
                    }
                } else {
                    let b = b - n;

                    if a < n {
                        let tmp = first[a];
                        first[a] = second[b];
                        second[b] = tmp;
                    } else {
                        let a = a - n;

                        let tmp = second[a];
                        second[a] = second[b];
                        second[b] = tmp;
                    }
                }
            }
            _ => {
                reversed = !reversed;
            }
        }
    }

    let first_text: String = first.iter().collect();
    let second_text: String = second.iter().collect();

    if reversed {
        println!("{}{}", second_text, first_text);
    } else {
        println!("{}{}", first_text, second_text);
    }
}
