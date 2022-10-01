use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let text = format!("{:02x}", n);

    println!("{}", text.to_uppercase());
}
