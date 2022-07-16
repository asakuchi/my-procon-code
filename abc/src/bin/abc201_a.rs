use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut a: [usize; 3],
    }

    a.sort();

    if a[2] - a[1] == a[1] - a[0] {
        println!("Yes");
    } else {
        println!("No");
    }
}
