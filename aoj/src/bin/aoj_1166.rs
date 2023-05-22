use std::collections::VecDeque;

fn main() {
    loop {
        let (w, h): (usize, usize) = input_tuple();

        if (w, h) == (0, 0) {
            break;
        }

        let mut list = Vec::new();

        for i in 0..h * 2 - 1 {
            let mut line = input_char_vec();

            if i % 2 == 0 {
                line.push(' ');
            }

            list.push(line);
        }

        // println!("{:?}", list);

        solve(h, w, &list);
    }
}

fn solve(h: usize, w: usize, list: &Vec<Vec<char>>) {
    let patterns = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let start = (0, 0);
    let goal = (h - 1, w - 1);

    let mut visited = vec![vec![false; w]; h];

    let mut queue = VecDeque::new();
    queue.push_back((start, 1));
    visited[start.0][start.1] = true;

    while let Some((current, step)) = queue.pop_front() {
        // 終了条件
        if current == goal {
            println!("{}", step);
            return;
        }

        for pattern in &patterns {
            let next = (
                current.0 as isize + pattern.0,
                current.1 as isize + pattern.1,
            );

            if 0 > next.0 || next.0 >= h as isize || 0 > next.1 || next.1 >= w as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            if list[(current.0 as isize * 2 + pattern.0) as usize]
                [(current.1 as isize * 2 + pattern.1) as usize]
                == '1'
            {
                continue;
            }

            if visited[next.0][next.1] {
                continue;
            }

            // 次へ
            visited[next.0][next.1] = true;
            queue.push_back((next, step + 1));
        }
    }

    println!("0");
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

fn input_char_vec() -> Vec<char> {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let x = buf.chars().collect();

    x
}
