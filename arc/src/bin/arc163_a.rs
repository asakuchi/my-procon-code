use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        solve(n, &s);
    }
}

fn solve(n: usize, s: &Vec<char>) {
    for i in 1..n {
        if s[i] > s[0] {
            println!("Yes");
            return;
        }
    }

    for i in 1..n {
        if s[i] == s[0] {
            let mut ok = true;

            for j in 0..i {
                if i + j >= n {
                    // NG
                    ok = false;
                    break;
                }

                if s[j] < s[i + j] {
                    println!("Yes");
                    return;
                }

                if s[j] > s[i + j] {
                    ok = false;
                    break;
                }
            }

            // 0~i と i~2*iが同じだった
            if 2 * i >= n {
                ok = false;
            }

            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
