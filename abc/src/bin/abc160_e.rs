use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut p: [usize; a],
        mut q: [usize; b],
        mut r: [usize; c],
    }

    p.sort();
    q.sort();
    r.sort();

    // とりあえず、pからx個、qからy個選ぶ
    let mut set = BTreeSet::new();

    let mut result = 0;

    for i in 0..x {
        let value = p.pop().unwrap();
        set.insert((value, i));
        result += value;
    }

    for i in 0..y {
        let value = q.pop().unwrap();
        set.insert((value, i + 2_000_00));
        result += value;
    }

    let mut uniq = 4_000_01;

    // println!("{:?}", set);

    while let Some(colorless) = r.pop() {
        let first = set.iter().next().copied().unwrap();

        // println!("colorless {} last {:?}", colorless, first);

        if colorless > first.0 {
            result += colorless;
            result -= first.0;

            set.remove(&first);
            set.insert((colorless, uniq));
            uniq += 1;
        } else {
            break;
        }
    }

    println!("{}", result);
}
