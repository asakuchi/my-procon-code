use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    for crane in 0..=x {
        let turtle = x - crane;

        if crane * 2 + turtle * 4 == y {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
