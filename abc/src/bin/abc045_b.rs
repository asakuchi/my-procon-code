use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s_a: Chars,
        mut s_b: Chars,
        mut s_c: Chars,
    }

    s_a.reverse();
    s_b.reverse();
    s_c.reverse();

    let mut next = 'a';

    loop {
        if let Some(value) = match next {
            'a' => s_a.pop(),
            'b' => s_b.pop(),
            _ => s_c.pop(),
        } {
            next = value;
        } else {
            println!("{}", next.to_uppercase());
            return;
        }
    }
}
