// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        answer: String,
    }

    let all = (1..=16).collect::<Vec<_>>();

    let cards = vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1, 2, 3, 4, 9, 10, 11, 12],
        vec![1, 2, 5, 6, 9, 10, 13, 14],
        vec![1, 3, 5, 7, 9, 11, 13, 15],
    ];

    let mut candidates = all.clone();

    for (yn, card) in answer.chars().zip(cards.iter()) {
        if yn == 'Y' {
            let not = all
                .iter()
                .filter(|&x| !card.contains(x))
                .collect::<Vec<_>>();

            candidates = candidates
                .iter()
                .filter(|x| !not.contains(x))
                .map(|x| *x)
                .collect::<Vec<_>>();
        } else {
            candidates = candidates
                .iter()
                .filter(|x| !card.contains(x))
                .map(|x| *x)
                .collect();
        }
    }

    if let Some(number) = candidates.iter().next() {
        println!("{}", number);
    }
}
