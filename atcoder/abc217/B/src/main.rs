use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: [String; 3],
    }

    let mut set = std::collections::HashSet::new();
    set.insert("ABC");
    set.insert("ARC");
    set.insert("AGC");
    set.insert("AHC");

    for contest in s {
        set.remove(contest.as_str());
    }

    println!("{}", set.iter().next().unwrap());
}
