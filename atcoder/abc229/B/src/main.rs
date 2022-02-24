use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    while a != 0 || b != 0 {
        let mod_a = a % 10;
        let mod_b = b % 10;

        if mod_a + mod_b >= 10 {
            println!("Hard");
            return;
        }

        a /= 10;
        b /= 10;
    }

    println!("Easy");
}
