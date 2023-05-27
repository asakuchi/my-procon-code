use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: usize,
        k: usize,
        s: Chars,
        x_y: [(isize, isize); m],
    }

    let mut items = HashSet::new();

    for &(x, y) in &x_y {
        items.insert((x, y));
    }

    let mut hp = h;

    let mut current = (0_isize, 0_isize);

    for i in 0..n {
        if hp == 0 {
            println!("No");
            return;
        }

        hp -= 1;

        if s[i] == 'R' {
            current.0 += 1;
        } else if s[i] == 'L' {
            current.0 -= 1;
        } else if s[i] == 'U' {
            current.1 += 1;
        } else {
            current.1 -= 1;
        }

        if items.contains(&current) {
            if hp < k {
                hp = k;
                items.remove(&current);
            }
        }
    }

    println!("Yes");
}
