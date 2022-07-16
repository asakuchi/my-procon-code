use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: [String; 4],
    }

    if s.contains(&String::from("H"))
        && s.contains(&String::from("2B"))
        && s.contains(&String::from("3B"))
        && s.contains(&String::from("HR"))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
