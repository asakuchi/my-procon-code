use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    if a > k {
        println!("{} {}", a - k, b);
    } else if a + b > k {
        println!("{} {}", 0, a + b - k);
    } else {
        println!("{} {}", 0, 0);
    }
}
