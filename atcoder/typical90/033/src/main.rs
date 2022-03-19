use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }

    if h == 1 || w == 1 {
        println!("{}", h * w);
    } else {
        println!("{}", (h / 2 + h % 2) * (w / 2 + w % 2));
    }
}
