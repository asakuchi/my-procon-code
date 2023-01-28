use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut list: Vec<_> = s.iter().enumerate().map(|(i, text)| (text, i)).collect();

    list.sort();

    let mut score = vec![0; n];

    'outer: for i in 1..n {
        let m = list[i].0.len().min(list[i - 1].0.len());

        for j in 0..m {
            if list[i].0[j] != list[i - 1].0[j] {
                score[list[i].1] = score[list[i].1].max(j);
                score[list[i - 1].1] = score[list[i - 1].1].max(j);

                continue 'outer;
            }
        }

        score[list[i].1] = score[list[i].1].max(m);
        score[list[i - 1].1] = score[list[i - 1].1].max(m);
    }

    let text = score.iter().format(" ");

    println!("{}", text);
}
