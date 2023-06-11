use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n % 5 == 0 || n % 5 == 1 || n % 5 == 2 {
        println!("{}", n - n % 5);
    } else {
        println!("{}", n + 5 - (n % 5));
    }
}
