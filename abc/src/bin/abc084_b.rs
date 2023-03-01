use proconio::input;
use regex::Regex;

fn main() {
    input! {
        a: usize,
        b: usize,
        s: String,
    }

    let pattern = format!(r"^[0-9]{{{}}}-[0-9]{{{}}}$", a, b);

    let re = Regex::new(pattern.as_str()).unwrap();

    if re.is_match(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
