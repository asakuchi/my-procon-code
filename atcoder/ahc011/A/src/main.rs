fn main() {
    let (n, t, mut tiles) = input();

    // println!("{:?}", tiles);

    let mut result = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if tiles[i][j] == '0' {
                if i <= n - 2 {
                    result.push("D");
                } else {
                    result.push("U");
                }
            }
        }
    }

    println!("{}", result.join(""));
}

fn input() -> (usize, usize, Vec<Vec<char>>) {
    use std::io;

    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let t: usize = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<Vec<_>> = Vec::new();

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.chars();

        let line: Vec<_> = iter
            // .map(|c| c.to_string().parse().unwrap())
            // .map(|x: usize| num::FromPrimitive::from_usize(x).unwrap())
            .collect();

        a.push(line);
    }

    (n, t, a)
}

fn output(result: &Vec<Vec<char>>) {
    for i in 0..30 {
        for j in 0..30 {
            print!("{}", result[i][j]);
        }
    }

    println!();
}
