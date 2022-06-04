use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        mut n: Chars,
    }

    let keta_1 = n.pop().unwrap();
    let keta_10 = n.pop().unwrap();

    println!("{}{}", keta_10, keta_1);
}
