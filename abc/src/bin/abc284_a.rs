use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [String; n],
    }

    for i in (0..n).rev() {
        println!("{}", a[i]);
    }
}
