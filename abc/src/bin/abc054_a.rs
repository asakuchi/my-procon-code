use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a == b {
        println!("Draw");
        return;
    }

    if a == 1 {
        println!("Alice");
        return;
    }

    if b == 1 {
        println!("Bob");
        return;
    }

    if a > b {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
