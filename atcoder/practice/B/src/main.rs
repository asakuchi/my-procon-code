use std::cmp::Ordering;
use std::io;
use std::io::{stdout, Write};

fn main() {
    // ----------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let _q: usize = iter.next().unwrap().parse().unwrap();

    // ----------------------

    let mut list = Vec::with_capacity(n);

    for c in 'A' as u8..'A' as u8 + n as u8 {
        list.push(c as char);
    }

    let mut count = 0;

    list.sort_by(|a, b| {
        println!("? {} {}", a, b);
        stdout().flush().unwrap();

        // ----------------------
        let stdin = io::stdin();

        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let ans = buf.trim_end().to_owned();
        // ----------------------

        count += 1;

        match ans.as_str() {
            ">" => Ordering::Greater,
            _ => Ordering::Less,
        }
    });

    eprintln!("count:{}", count);

    print!("! ");

    for c in list.iter() {
        print!("{}", c);
    }

    println!();
}
