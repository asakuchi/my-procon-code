use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n * 2],
    }

    let mut rank = Vec::with_capacity(n * 2);

    for i in 0..2 * n {
        // 勝利数、番号
        rank.push((0, i));
    }

    for round in 0..m {
        let mut iteration = rank.iter_mut();

        for _ in 0..n {
            let mut p1 = iteration.next().unwrap();
            let mut p2 = iteration.next().unwrap();

            match a[p1.1][round] {
                'G' => match a[p2.1][round] {
                    'G' => {}
                    'C' => {
                        p1.0 += 1;
                    }
                    _ => {
                        p2.0 += 1;
                    }
                },
                'C' => match a[p2.1][round] {
                    'G' => {
                        p2.0 += 1;
                    }
                    'C' => {}
                    _ => {
                        p1.0 += 1;
                    }
                },
                _ => match a[p2.1][round] {
                    'G' => {
                        p1.0 += 1;
                    }
                    'C' => {
                        p2.0 += 1;
                    }
                    _ => {}
                },
            }
        }

        rank.sort_by_key(|&(win, num)| (Reverse(win), num));
    }

    for &(_win, num) in rank.iter() {
        println!("{}", num + 1);
    }
}
