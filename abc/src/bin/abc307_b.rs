use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if kaibun(&s[i], &s[j]) {
                println!("Yes");

                return;
            }
        }
    }

    println!("No");
}

fn kaibun(a: &Vec<char>, b: &Vec<char>) -> bool {
    let mut joined = Vec::new();

    for &c in a.iter() {
        joined.push(c);
    }

    for &c in b.iter() {
        joined.push(c);
    }

    for i in 0..joined.len() / 2 {
        if joined[i] != joined[joined.len() - 1 - i] {
            return false;
        }
    }

    true
}
