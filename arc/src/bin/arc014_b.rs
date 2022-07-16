use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        w: [Chars; n],
    }

    let mut is_taka_turn = false;

    let mut last_word = w[0].last().unwrap();

    let mut set = std::collections::HashSet::new();
    set.insert(w[0].clone());

    for i in 1..n {
        if w[i][0] != *last_word || set.contains(&w[i]) {
            if is_taka_turn {
                println!("LOSE");
            } else {
                println!("WIN");
            }

            return;
        }

        last_word = w[i].last().unwrap();
        set.insert(w[i].clone());
        is_taka_turn = !is_taka_turn;
    }

    println!("DRAW");
}
