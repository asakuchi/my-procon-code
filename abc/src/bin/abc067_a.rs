use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}
