use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [String; n],
    }

    let mut set = std::collections::HashSet::new();

    for text in a {
        if &text[0..1] == "!" {
            let other = &text[1..];

            if set.contains(other) {
                println!("{}", other);
                return;
            }
        } else {
            if set.contains(&format!("!{}", text)) {
                println!("{}", text);
                return;
            }
        }

        set.insert(text);
    }

    println!("satisfiable",);
}
