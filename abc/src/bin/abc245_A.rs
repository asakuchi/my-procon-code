use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a < c {
        println!("Takahashi");
        return;
    }

    if a == c {
        if b <= d {
            println!("Takahashi");
            return;
        } else {
            println!("Aoki");
            return;
        }
    }

    println!("Aoki");
}
