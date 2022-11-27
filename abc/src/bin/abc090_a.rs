use proconio::{input, marker::Chars};

fn main() {
    input! {
        c: [Chars; 3],
    }

    print!("{}", c[0][0]);
    print!("{}", c[1][1]);
    print!("{}", c[2][2]);
    println!();
}
