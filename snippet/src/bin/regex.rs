use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let re = Regex::new(r"^[A-Z][1-9]\d{5}[A-Z]$").unwrap();

    if re.is_match(&s) {
        println!("Yes");
    } else {
        println!("No");
    }

    // -------------------

    // 正規表現の中に変数を使いたい場合
    // "{" をエスケープするときは "{{" と書く

    let a = 3;
    let b = 4;

    let pattern = format!(r"^[0-9]{{{}}}-[0-9]{{{}}}$", a, b);

    let re = Regex::new(pattern.as_str()).unwrap();

    if re.is_match(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
