use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    // 6 -> 5 -> 6 を繰り返す

    let quo = x / 11;
    let rem = x % 11;

    if rem == 0 {
        println!("{}", quo * 2);
    } else if rem <= 6 {
        println!("{}", quo * 2 + 1);
    } else {
        println!("{}", quo * 2 + 2);
    }
}
