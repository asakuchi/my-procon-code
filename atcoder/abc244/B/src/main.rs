use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        t: Chars,

    }

    let mut position = (0, 0);
    let mut direction = (1, 0);

    for c in t.iter() {
        match c {
            'S' => {
                position.0 += direction.0;
                position.1 += direction.1;
            }
            _ => {
                if direction.0 == 1 {
                    direction.0 = 0;
                    direction.1 = -1;
                } else if direction.1 == -1 {
                    direction.0 = -1;
                    direction.1 = 0;
                } else if direction.0 == -1 {
                    direction.0 = 0;
                    direction.1 = 1;
                } else if direction.1 == 1 {
                    direction.0 = 1;
                    direction.1 = 0;
                }
            }
        }
    }

    println!("{} {}", position.0, position.1);
}
