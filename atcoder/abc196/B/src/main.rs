use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: String,
    }

    let result = x.split(".").collect::<Vec<_>>()[0];

    println!("{}", result);
}
