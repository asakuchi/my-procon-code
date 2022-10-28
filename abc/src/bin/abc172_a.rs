use proconio::input;

fn main() {
    input! {
        a: usize
    }

    println!("{}", a + a * a + a * a * a);
}
