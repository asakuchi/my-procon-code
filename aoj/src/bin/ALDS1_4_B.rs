use std::io;

fn main() {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let mut s: Vec<isize> = iter.map(|x| x.parse().unwrap()).collect();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let _q: usize = buf.parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let t: Vec<isize> = iter.map(|x| x.parse().unwrap()).collect();

    // ------------------------------------

    s.sort();

    let mut result = 0;

    for value in t {
        let mut high = n;
        let mut low = 0;
        let mut mid;

        loop {
            mid = (high + low) / 2;

            if value == s[mid] {
                result += 1;
                break;
            }

            if mid == low {
                break;
            }

            if value > s[mid] {
                low = mid;
            } else {
                high = mid;
            }
        }
    }

    println!("{}", result);
}
