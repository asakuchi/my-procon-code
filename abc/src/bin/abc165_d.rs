use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    }

    println!("{}", (a * (b - 1).min(n)) / b - a * ((b - 1).min(n) / b));
}
