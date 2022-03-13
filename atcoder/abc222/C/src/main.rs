use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n * 2],
    }

    let mut rank = BTreeSet::new();

    for i in 0..2 * n {
        // 勝利数、番号
        rank.insert((Reverse(0), i));
    }

    for round in 0..m {
        let mut next_rank = BTreeSet::new();

        let mut iteration = rank.iter();

        while iteration.len() > 0 {
            let p1 = iteration.next().unwrap();
            let p2 = iteration.next().unwrap();

            let p1_next;
            let p2_next;

            match a[p1.1][round] {
                'G' => match a[p2.1][round] {
                    'G' => {
                        p1_next = (p1.0, p1.1);
                        p2_next = (p2.0, p2.1);
                    }
                    'C' => {
                        p1_next = (Reverse((p1.0).0 + 1), p1.1);
                        p2_next = (p2.0, p2.1);
                    }
                    _ => {
                        p1_next = (p1.0, p1.1);
                        p2_next = (Reverse((p2.0).0 + 1), p2.1);
                    }
                },
                'C' => match a[p2.1][round] {
                    'G' => {
                        p1_next = (p1.0, p1.1);
                        p2_next = (Reverse((p2.0).0 + 1), p2.1);
                    }
                    'C' => {
                        p1_next = (p1.0, p1.1);
                        p2_next = (p2.0, p2.1);
                    }
                    _ => {
                        p1_next = (Reverse((p1.0).0 + 1), p1.1);
                        p2_next = (p2.0, p2.1);
                    }
                },
                _ => match a[p2.1][round] {
                    'G' => {
                        p1_next = (Reverse((p1.0).0 + 1), p1.1);
                        p2_next = (p2.0, p2.1);
                    }
                    'C' => {
                        p1_next = (p1.0, p1.1);
                        p2_next = (Reverse((p2.0).0 + 1), p2.1);
                    }
                    _ => {
                        p1_next = (p1.0, p1.1);
                        p2_next = (p2.0, p2.1);
                    }
                },
            }

            next_rank.insert(p1_next);
            next_rank.insert(p2_next);
        }

        rank = next_rank;

        // println!("{:?}", rank);
    }

    for &(_win, num) in rank.iter() {
        println!("{}", num + 1);
    }
}
