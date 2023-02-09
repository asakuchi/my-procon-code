use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let text = format!("{}{}", a, b);

    let x: usize = text.parse().unwrap();

    for i in 1..=10_000 {
        if x == i * i {
            println!("Yes");
            return;
        }

        if x < i * i {
            break;
        }
    }

    println!("No");
}
