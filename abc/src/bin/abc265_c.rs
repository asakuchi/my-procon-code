use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut visited = vec![vec![false; w]; h];

    let mut current = (0, 0);

    loop {
        let g = a[current.0][current.1];

        if visited[current.0][current.1] {
            println!("-1");
            return;
        }

        visited[current.0][current.1] = true;

        if g == 'U' {
            if current.0 == 0 {
                println!("{} {}", current.0 + 1, current.1 + 1);
                return;
            }

            current.0 -= 1;
        } else if g == 'D' {
            if current.0 == h - 1 {
                println!("{} {}", current.0 + 1, current.1 + 1);
                return;
            }

            current.0 += 1;
        } else if g == 'L' {
            if current.1 == 0 {
                println!("{} {}", current.0 + 1, current.1 + 1);
                return;
            }

            current.1 -= 1;
        } else if g == 'R' {
            if current.1 == w - 1 {
                println!("{} {}", current.0 + 1, current.1 + 1);
                return;
            }

            current.1 += 1;
        }
    }
}
