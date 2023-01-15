use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if b / 2 == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
