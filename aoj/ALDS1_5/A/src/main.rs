use std::io;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let a: Vec<usize> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let _q: usize = buf.parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let m: Vec<usize> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // ------------------------------------

    let mut result = vec![false; 2001]; // 1 <= mi <= 2000

    for bit in 0..(1 << n) {
        let sum = (0..n).fold(0, |sum, i| sum + if bit & (1 << i) > 0 { a[i] } else { 0 });

        result[sum] = true;
    }

    for mi in m {
        println!("{}", if result[mi] { "yes" } else { "no" });
    }
}
