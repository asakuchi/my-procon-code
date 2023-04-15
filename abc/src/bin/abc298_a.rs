use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    if s.contains("o") && !s.contains("x") {
        println!("Yes");
    } else {
        println!("No");
    }
}
