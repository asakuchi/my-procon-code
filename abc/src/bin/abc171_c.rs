use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }

    let mut list = Vec::new();

    while n != 0 {
        n -= 1;

        list.push(n % 26);
        n /= 26;
    }

    list.reverse();

    for &c in &list {
        print!("{}", (('a' as u8) + c as u8) as char);
    }

    println!();
}
