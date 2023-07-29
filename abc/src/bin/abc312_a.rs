use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let list = vec!["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

    if list.contains(&s.as_str()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
