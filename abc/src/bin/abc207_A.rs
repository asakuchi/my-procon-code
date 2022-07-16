use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut num: [usize; 3]
    }

    num.sort();

    println!("{}", num[1] + num[2]);
}
