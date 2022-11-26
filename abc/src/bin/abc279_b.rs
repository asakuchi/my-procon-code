use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,

    }

    if s.len() < t.len() {
        println!("No");
        return;
    }

    for i in 0..s.len() - t.len() + 1 {
        let mut ok = true;

        for j in 0..t.len() {
            if s[i + j] != t[j] {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
