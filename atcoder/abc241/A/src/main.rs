use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: [usize; 10],
    }

    let mut num = 0;

    for _ in 0..3 {
        num = a[num];
    }

    println!("{}", num);
}
