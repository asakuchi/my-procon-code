use proconio::input;

fn main() {
    input! {
        x: isize,
    }

    if x >= 0 {
        println!("{}", x);
    } else {
        println!("0");
    }
}
