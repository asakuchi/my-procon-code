use std::io;

fn main() {
    let stdin = io::stdin();

    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let n: usize = iter.next().unwrap().parse().unwrap();
        let x: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 && x == 0 {
            // 終了
            return;
        }

        /* ---------------------------------- */

        let mut result = 0;

        for i in 1..=n {
            for j in i + 1..=n {
                for k in j + 1..=n {
                    if i + j + k == x {
                        result += 1;
                    }
                }
            }
        }

        println!("{}", result);
    }
}
