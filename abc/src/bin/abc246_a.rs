use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        p1: (isize, isize),
        p2: (isize, isize),
        p3: (isize, isize),
    }

    let x = if p1.0 == p2.0 {
        p3.0
    } else if p1.0 == p3.0 {
        p2.0
    } else {
        p1.0
    };

    let y = if p1.1 == p2.1 {
        p3.1
    } else if p1.1 == p3.1 {
        p2.1
    } else {
        p1.1
    };

    println!("{} {}", x, y);
}
