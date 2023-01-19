use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut n: isize,
    }

    if n == 0 {
        println!("0");
        return;
    }

    let mut result = Vec::new();

    while n != 0 {
        let mut p = n / (-2);
        let mut q = n % (-2);

        if q == -1 {
            p += 1;
            q += 2;
        }

        // println!("{} / (-2) = {}", n, p);
        // println!("{} % (-2) = {}", n, q);

        result.push(q);

        n = p;
    }

    result.reverse();

    // println!("{:?}", result);

    let text = result.iter().format("");

    println!("{}", text);
}
