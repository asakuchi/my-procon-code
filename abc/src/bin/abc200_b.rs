use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }

    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = format!("{}200", n.to_string(),).parse().unwrap();
        }
    }

    println!("{}", n);
}
