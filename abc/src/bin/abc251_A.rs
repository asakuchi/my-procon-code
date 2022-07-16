use proconio::input;

fn main() {
    input! {
        s: String,

    }

    if s.len() == 1 {
        println!("{}{}{}{}{}{}", s, s, s, s, s, s);
    } else if s.len() == 2 {
        println!("{}{}{}", s, s, s);
    } else {
        println!("{}{}", s, s);
    }
}
