use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut pre = vec![false; t.len() + 1];
    let mut post = vec![false; t.len() + 1];

    pre[0] = true;
    post[t.len()] = true;

    for i in 0..t.len() {
        if s[i] == t[i] || s[i] == '?' || t[i] == '?' {
            pre[i + 1] = true;
        } else {
            break;
        }
    }

    for i in 0..t.len() {
        if s[s.len() - 1 - i] == t[t.len() - 1 - i]
            || s[s.len() - 1 - i] == '?'
            || t[t.len() - 1 - i] == '?'
        {
            post[t.len() - 1 - i] = true;
        } else {
            break;
        }
    }

    for i in 0..=t.len() {
        if pre[i] && post[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
