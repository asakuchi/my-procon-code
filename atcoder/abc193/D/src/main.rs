use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    }

    let mut taka = HashMap::new();
    let mut aoki = HashMap::new();
    let mut common = HashMap::new();

    for i in 0..4 {
        *taka.entry(s[i].to_string().parse().unwrap()).or_insert(0) += 1;
        *common.entry(s[i].to_string().parse().unwrap()).or_insert(0) += 1;

        *aoki.entry(t[i].to_string().parse().unwrap()).or_insert(0) += 1;
        *common.entry(t[i].to_string().parse().unwrap()).or_insert(0) += 1;
    }

    let mut win_count = 0;

    for i in 1..=9 {
        for j in 1..=9 {
            if (i == j && *common.entry(i).or_insert(0) + 2 > k)
                || *common.entry(i).or_insert(0) + 1 > k
                || *common.entry(j).or_insert(0) + 1 > k
            {
                continue;
            }

            let taka_score: usize = (1..=9)
                .map(|x| x * 10usize.pow(*taka.entry(x).or_insert(0) + if x == i { 1 } else { 0 }))
                .sum();

            let aoki_score: usize = (1..=9)
                .map(|x| x * 10usize.pow(*aoki.entry(x).or_insert(0) + if x == j { 1 } else { 0 }))
                .sum();

            if taka_score > aoki_score {
                if i != j {
                    win_count +=
                        (k - *common.entry(i).or_insert(0)) * (k - *common.entry(j).or_insert(0));
                } else {
                    win_count += (k - *common.entry(i).or_insert(0))
                        * (k - *common.entry(i).or_insert(0) - 1);
                }
            }
        }
    }

    println!("{}", win_count as f64 / ((9 * k - 8) * (9 * k - 9)) as f64);
}
