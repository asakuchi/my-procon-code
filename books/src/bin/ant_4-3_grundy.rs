use std::collections::{HashMap, HashSet};

fn main() {
    let w = 2;
    let h = 2;

    solve(w, h);

    let w = 4;
    let h = 2;

    solve(w, h);

    let w = 5;
    let h = 8;

    solve(w, h);
}

fn solve(w: usize, h: usize) {
    let mut memo = HashMap::new();

    let result = grundy(w, h, &mut memo);

    if result != 0 {
        println!("WIN");
    } else {
        println!("LOSE");
    }
}

fn grundy(w: usize, h: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&value) = memo.get(&(w, h)) {
        return value;
    }

    if w == 0 && h == 0 {
        return 0;
    }

    let mut set = HashSet::new();

    for i in 2..w - 1 {
        set.insert(grundy(i, h, memo) ^ grundy(w - i, h, memo));
    }

    for i in 2..h - 1 {
        set.insert(grundy(w, i, memo) ^ grundy(w, h - i, memo));
    }

    let mut g = 0;

    while set.contains(&g) {
        g += 1;
    }

    memo.insert((w, h), g);

    println!("w {} h {} g {}", w, h, g);

    g
}
