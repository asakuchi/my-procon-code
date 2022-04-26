use proconio::input;

fn main() {
    input! {
        a: [u8; 26],
    }

    for num in a {
        print!("{}", ('a' as u8 + num - 1) as char);
    }

    println!();
}
