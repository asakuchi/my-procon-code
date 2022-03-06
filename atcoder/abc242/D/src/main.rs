use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
        mut tk: [(usize, Usize1); q],
    }

    for &(t, k) in tk.iter() {
        let current_char = st_k(&s, t, k);

        println!("{}", current_char);
    }
}

fn st_k(s: &Vec<char>, t: usize, k: usize) -> char {
    if k == 0 {
        let mut c = s[0];

        for _ in 0..t % 3 {
            c = (c as u8 + 1) as char;

            if c > 'C' {
                c = 'A';
            }
        }

        return c;
    }

    if t == 0 {
        return s[k];
    }

    let parent = st_k(s, t - 1, k / 2);

    let current_char;

    if k % 2 == 0 {
        match parent {
            'A' => {
                current_char = 'B';
            }
            'B' => {
                current_char = 'C';
            }
            _ => {
                current_char = 'A';
            }
        }
    } else {
        match parent {
            'A' => {
                current_char = 'C';
            }
            'B' => {
                current_char = 'A';
            }
            _ => {
                current_char = 'B';
            }
        }
    }

    current_char
}
