use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    let mut taka_max = -1_000_000;

    for taka in 0..n {
        let mut aoki_max = -1_000_000;
        let mut taka_temp_max = -1_000_000;

        for aoki in 0..n {
            if taka == aoki {
                continue;
            }

            let start = if taka < aoki { taka } else { aoki };

            let end = if taka < aoki { aoki } else { taka };

            let list = &a[start..=end];

            let mut taka_score = 0;
            let mut aoki_score = 0;

            for i in 0..list.len() {
                if i % 2 == 0 {
                    taka_score += list[i];
                } else {
                    aoki_score += list[i];
                }
            }

            if aoki_score > aoki_max {
                aoki_max = aoki_score;
                taka_temp_max = taka_score;
            }
        }

        taka_max = taka_max.max(taka_temp_max);
    }

    println!("{}", taka_max);
}
