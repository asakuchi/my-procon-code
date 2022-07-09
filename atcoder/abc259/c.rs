use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_queue = VecDeque::new();
    let mut t_queue = VecDeque::new();

    for i in 0..s.len() {
        if let Some((c, count)) = s_queue.pop_back() {
            if c == s[i] {
                s_queue.push_back((s[i], count + 1));
            } else {
                s_queue.push_back((c, count));
                s_queue.push_back((s[i], 1));
            }
        } else {
            s_queue.push_back((s[i], 1));
        }
    }

    for i in 0..t.len() {
        if let Some((c, count)) = t_queue.pop_back() {
            if c == t[i] {
                t_queue.push_back((t[i], count + 1));
            } else {
                t_queue.push_back((c, count));
                t_queue.push_back((t[i], 1));
            }
        } else {
            t_queue.push_back((t[i], 1));
        }
    }

    while let Some((s_c, s_count)) = s_queue.pop_front() {
        if let Some((t_c, t_count)) = t_queue.pop_front() {
            if s_c == t_c {
                if s_count == t_count {
                    continue;
                }

                if s_count >= 2 && t_count >= s_count {
                    continue;
                }

                println!("No");
                return;
            } else {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
    }

    if t_queue.len() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
