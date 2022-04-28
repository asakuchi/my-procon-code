use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }

    let mut list = Vec::new();

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
            list.push('B');
        } else {
            n -= 1;
            list.push('A');
        }
    }

    list.push('A');

    list.reverse();

    for c in list {
        print!("{}", c);
    }

    println!();
}
