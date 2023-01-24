use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let product = 2025 - n;

    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == product {
                println!("{} x {}", i, j);
            }
        }
    }
}
