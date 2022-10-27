use proconio::input;

fn main() {
    input! {
        mut x: usize
    }

    let coin_500 = x / 500;

    x %= 500;

    let coin_5 = x / 5;

    println!("{}", coin_500 * 1_000 + coin_5 * 5);
}
