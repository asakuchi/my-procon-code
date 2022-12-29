use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    if x < 1200 {
        println!("ABC");
    } else {
        println!("ARC");
    }
}
