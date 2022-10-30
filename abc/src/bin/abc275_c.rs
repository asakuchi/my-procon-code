use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 9],
    }

    let mut result = 0;

    for i in 0isize..9 {
        for j in 0isize..9 {
            if s[i as usize][j as usize] != '#' {
                continue;
            }

            // (i,j) を左上の頂点とする正方形を探す
            for k in 0isize..9 {
                for l in 0isize..9 {
                    if s[k as usize][l as usize] != '#' {
                        continue;
                    }

                    if i == k && j == l {
                        continue;
                    }

                    // (k, l) を 次の頂点とする正方形

                    let p3 = (k + l - j, l - k + i);
                    let p4 = (i + l - j, j - k + i);

                    if p3.0 < 0
                        || p3.0 >= 9
                        || p3.1 < 0
                        || p3.1 >= 9
                        || p4.0 < 0
                        || p4.0 >= 9
                        || p4.1 < 0
                        || p4.1 >= 9
                    {
                        continue;
                    }

                    if s[p3.0 as usize][p3.1 as usize] != '#' {
                        continue;
                    }

                    if s[p4.0 as usize][p4.1 as usize] != '#' {
                        continue;
                    }

                    result += 1;
                }
            }
        }
    }

    println!("{}", result / 4);
}
