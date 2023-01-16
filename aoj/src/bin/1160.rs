fn main() {
    loop {
        let (w, h) = input_tuple();

        if w == 0 && h == 0 {
            break;
        }

        let c = input_vec_2d(h);

        let mut visited = vec![vec![false; w]; h];

        let mut result = 0;

        for i in 0..h {
            for j in 0..w {
                if c[i][j] == 0 {
                    continue;
                }

                if visited[i][j] {
                    continue;
                }

                result += 1;

                rec(h, w, &c, i, j, &mut visited);
            }
        }

        println!("{}", result);
    }
}

fn input_tuple<T>() -> (T, T)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();

    (n, m)
}

fn input_vec_2d<T>(n: usize) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // 二次元ベクタ

    let stdin = std::io::stdin();

    let mut a = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.split_whitespace();

        let line = iter.map(|x| x.parse().unwrap()).collect();

        a.push(line);
    }

    a
}

fn rec(h: usize, w: usize, c: &Vec<Vec<usize>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) {
    let directions = vec![
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    for direct in directions {
        let next_i = i as isize + direct.0;
        let next_j = j as isize + direct.1;

        if next_i < 0 || next_i >= h as isize || next_j < 0 || next_j >= w as isize {
            continue;
        }

        let next_i = next_i as usize;
        let next_j = next_j as usize;

        if c[next_i][next_j] == 0 {
            continue;
        }

        if visited[next_i][next_j] {
            continue;
        }

        visited[next_i][next_j] = true;

        rec(h, w, c, next_i, next_j, visited);
    }
}
