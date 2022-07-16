use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a * a + b * b < c * c {
        println!("Yes");
    } else {
        println!("No");
    }
}
