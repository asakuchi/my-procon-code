use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    prev_permutation(&mut a);

    println!("{}", a.iter().format(" "));
}

fn prev_permutation(list: &mut Vec<usize>) {
    let n = list.len();

    let mut set = BTreeSet::new();

    for i in (0..n).rev() {
        let target = list[i];
        set.insert(target);

        if let Some(&last) = set.range(..target).last() {
            set.remove(&last);

            list[i] = last;

            let mut j = i + 1;

            for &value in set.iter().rev() {
                list[j] = value;
                j += 1;
            }

            break;
        }
    }
}
