use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut card_num_in_box = vec![BTreeSet::new(); n + 10];
    let mut box_num_containing_card = vec![BTreeSet::new(); 200_010];

    for uniq_num in 0..q {
        input! {
            c: usize,
        }

        if c == 1 {
            input! {
                i: usize,
                j: usize,
            }

            // 箱jにカードiを入れる
            card_num_in_box[j].insert((i, uniq_num));
            box_num_containing_card[i].insert(j);
        } else if c == 2 {
            input! {
                i: usize,
            }

            let text = card_num_in_box[i]
                .iter()
                .map(|&(card_num, _)| card_num)
                .format(" ");
            println!("{}", text);
        } else {
            input! {
                i: usize,
            }

            let text = box_num_containing_card[i]
                .iter()
                .map(|&box_num| box_num)
                .format(" ");
            println!("{}", text);
        }
    }
}
