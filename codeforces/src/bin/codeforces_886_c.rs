fn main() {
    let t = input_value();

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    let mut grid = Vec::new();

    for _ in 0..8 {
        let line = input_char_vec();

        grid.push(line);
    }

    for i in 0..8 {
        for j in 0..8 {
            if grid[i][j] != '.' {
                let mut now_i = i;

                let mut result = Vec::new();

                while now_i < 8 && grid[now_i][j] != '.' {
                    result.push(grid[now_i][j]);
                    now_i += 1;
                }

                let text: String = result.iter().collect();
                println!("{}", text);

                return;
            }
        }
    }
}

fn input_value<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n = buf.parse().unwrap();

    n
}

fn input_char_vec() -> Vec<char> {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let x = buf.chars().collect();

    x
}
