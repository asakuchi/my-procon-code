use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut result = Vec::new();

    for i in 0..n - 8 {
        for j in 0..m - 8 {
            if s[i][j] == '#' {
                let mut ok = true;

                // 左上
                for x in 0..3 {
                    for y in 0..3 {
                        let next_x = i + x;
                        let next_y = j + y;

                        if s[next_x][next_y] != '#' {
                            ok = false;
                            break;
                        }
                    }
                }

                // 白
                for x in 0..3 {
                    if s[i + x][j + 3] != '.' {
                        ok = false;
                        break;
                    }
                }

                for y in 0..3 {
                    if s[i + 3][j + y] != '.' {
                        ok = false;
                        break;
                    }
                }

                if s[i + 3][j + 3] != '.' {
                    ok = false;
                }

                // 右下
                for x in 0..3 {
                    for y in 0..3 {
                        let next_x = i + x + 6;
                        let next_y = j + y + 6;

                        if s[next_x][next_y] != '#' {
                            ok = false;
                            break;
                        }
                    }
                }

                // 白
                for x in 0..3 {
                    if s[i + x + 5][j + 5] != '.' {
                        ok = false;
                        break;
                    }
                }

                for y in 0..3 {
                    if s[i + 5][j + y + 5] != '.' {
                        ok = false;
                        break;
                    }
                }

                if s[i + 5][j + 5] != '.' {
                    ok = false;
                }

                if ok {
                    result.push((i, j));
                }
            }
        }
    }

    result.sort();

    for (i, j) in result {
        println!("{} {}", i + 1, j + 1);
    }
}
