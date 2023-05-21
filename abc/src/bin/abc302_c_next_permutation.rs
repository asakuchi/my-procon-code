use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n],
    }

    s.sort();

    loop {
        let mut ok = true;

        for i in 0..n - 1 {
            let mut diff = 0;

            for j in 0..m {
                if s[i][j] != s[i + 1][j] {
                    diff += 1;
                }
            }

            if diff != 1 {
                ok = false;
                break;
            }
        }

        if ok {
            println!("Yes");
            return;
        }

        if !s.next_permutation() {
            break;
        }
    }

    println!("No");
}
