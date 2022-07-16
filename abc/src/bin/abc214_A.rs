use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n <= 125 {
        println!("4");
    } else if n <= 211 {
        println!("6");
    } else {
        println!("8");
    }
}
