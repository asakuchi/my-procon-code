use std::io;
use std::io::{stdout, Write};

fn main() {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    let mut list = vec![true; 2 * n + 1];

    for _ in 0..n + 1 {
        for taka in 0..2 * n + 1 {
            if list[taka] {
                println!("{}", taka + 1);
                stdout().flush().unwrap();
                list[taka] = false;
                break;
            }
        }

        let stdin = io::stdin();

        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let aoki: usize = buf.parse().unwrap();

        if aoki == 0 {
            return;
        }

        list[aoki - 1] = false;
    }
}
