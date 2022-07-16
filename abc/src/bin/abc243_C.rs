use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
        s: Chars,
    }

    let mut hash = std::collections::HashMap::new();
    let mut y_set = std::collections::HashSet::new();

    for (&(x, y), &c) in xy.iter().zip(s.iter()) {
        hash.entry((y, 'L')).or_insert(vec![0]);
        hash.entry((y, 'R')).or_insert(vec![1_000_000_001]);
        let entry = hash.entry((y, c)).or_insert(Vec::new());

        entry.push(x);

        y_set.insert(y);
    }

    for &y in y_set.iter() {
        let l_list = hash.get(&(y, 'L'));
        let r_list = hash.get(&(y, 'R'));

        let l_max = l_list.unwrap().iter().max().unwrap();
        let r_min = r_list.unwrap().iter().min().unwrap();

        if *r_min < *l_max {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
