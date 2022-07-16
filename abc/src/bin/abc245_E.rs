use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ItemType {
    Choco = 2,
    ChocoBox = 1,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; m],
        d: [usize; m],
    }

    let mut chocos: Vec<_> = (0..n)
        .map(|i| (Reverse(a[i]), ItemType::Choco, b[i]))
        .collect();
    let mut choco_boxes: Vec<_> = (0..m)
        .map(|i| (Reverse(c[i]), ItemType::ChocoBox, d[i]))
        .collect();

    let mut list = Vec::new();

    list.append(&mut chocos);
    list.append(&mut choco_boxes);

    list.sort();

    let mut d = std::collections::BTreeSet::new();

    for i in 0..list.len() {
        match list[i].1 {
            ItemType::ChocoBox => {
                d.insert((list[i].2, i));
            }
            _ => {
                let next = d.range((list[i].2, 0)..).next().copied();
                match next {
                    Some(value) => {
                        d.remove(&value);
                    }
                    None => {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }

    println!("Yes");
}
