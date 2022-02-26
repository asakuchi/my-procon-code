use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],

    }

    for i in 0..n {
        for j in 0..n {
            let mut count = 0;
            let mut sharp_count = 0;

            'yoko: for k in 0..6 {
                if i + k >= n {
                    continue 'yoko;
                }

                count += 1;
                if s[i + k][j] == '#' {
                    sharp_count += 1;
                }
            }

            if count >= 6 && sharp_count >= 4 {
                println!("Yes");
                return;
            }

            let mut count = 0;
            let mut sharp_count = 0;

            'tate: for k in 0..6 {
                if j + k >= n {
                    continue 'tate;
                }

                count += 1;
                if s[i][j + k] == '#' {
                    sharp_count += 1;
                }
            }

            if count >= 6 && sharp_count >= 4 {
                println!("Yes");
                return;
            }

            let mut count = 0;
            let mut sharp_count = 0;

            'naname: for k in 0..6 {
                if j + k >= n || i + k >= n {
                    continue 'naname;
                }

                count += 1;
                if s[i + k][j + k] == '#' {
                    sharp_count += 1;
                }
            }

            if count >= 6 && sharp_count >= 4 {
                println!("Yes");
                return;
            }

            let mut count = 0;
            let mut sharp_count = 0;

            'naname2: for k in 0..6 {
                if j + k >= n || (i as isize - k as isize) < 0 {
                    continue 'naname2;
                }

                count += 1;
                if s[i - k][j + k] == '#' {
                    sharp_count += 1;
                }
            }

            if count >= 6 && sharp_count >= 4 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
