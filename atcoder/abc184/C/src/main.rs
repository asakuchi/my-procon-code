use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        r1: isize,
        c1: isize,
        r2: isize,
        c2: isize,
    }

    if r1 == r2 && c1 == c2 {
        println!("0");
    } else if r1 + c1 == r2 + c2 {
        println!("1");
    } else if r1 - c1 == r2 - c2 {
        println!("1");
    } else if (r1 - r2).abs() + (c1 - c2).abs() <= 3 {
        println!("1");
    } else if ((r1 - r2).abs() + (c1 - c2).abs()) % 2 == 0 {
        println!("2");
    } else if (r1 + c1 - (r2 + c2)).abs() <= 3 {
        println!("2");
    } else if (r1 - c1 - (r2 - c2)).abs() <= 3 {
        println!("2");
    } else if (r1 - r2).abs() + (c1 - c2).abs() <= 6 {
        println!("2");
    } else {
        println!("3");
    }
}
