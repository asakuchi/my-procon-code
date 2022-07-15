use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        names: [(String, String); n],
    }

    let mut set = std::collections::HashSet::new();

    for name in names {
        if set.contains(&name) {
            println!("Yes");
            return;
        }

        set.insert(name);
    }

    println!("No");
}
