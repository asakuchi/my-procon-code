use proconio::input;

fn main() {
    input! {
        k: u8,
    }

    for c in 'A' as u8..'A' as u8 + k {
        print!("{}", c as char);
    }

    println!();
}
