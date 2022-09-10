use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,

    }

    if t.len() < s.len() {
        println!("No");
        return;
    }

    if s == t[..s.len()] {
        println!("Yes");
    } else {
        println!("No");
    }
}
