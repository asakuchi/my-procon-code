use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: usize
    }

    if k <= 12 {
        println!("{}", k);
        return;
    }

    let mut index = 13;

    let mut lunlun = vec![0; 11];
    lunlun[0] = 1;
    lunlun[1] = 2;

    let mut keta = 2;

    while index != k {
        println!("{:?}", lunlun);

        let (next, next_keta) = next(&lunlun, keta);

        lunlun = next;
        keta = next_keta;

        index += 1;
    }

    // println!("{:?}", lunlun);

    lunlun.reverse();

    let text = lunlun.iter().format("").to_string();
    let result: usize = text.parse().unwrap();

    println!("{}", result);
}

fn next(lunlun: &Vec<usize>, keta: usize) -> (Vec<usize>, usize) {
    let mut next = lunlun.clone();

    // let mut updated = false;

    let mut next_keta = keta;

    for i in 0..keta {
        // 上位の桁より小さい桁を探す
        if next[i + 1] >= next[i] && next[i] < 9 {
            // println!("keta {} kasan", i);

            next[i] += 1;

            let mut value = next[i] - 1;

            for j in (0..i).rev() {
                next[j] = value;

                if value > 0 {
                    value -= 1;
                }
            }

            break;
        }

        if i + 1 == keta {
            if next[i] == 9 {
                next_keta += 1;

                // println!("kuriagari keta {} kasan", i + 1);
                // println!("next_1 {:?}", next);

                next[i + 1] += 1;

                // println!("next_2 {:?}", next);

                for j in 0..=i {
                    next[j] = 0;

                    // println!("next_3 {:?}", next);
                }
            } else {
                // println!("i keta {} kasan when next 0", i);

                next[i] += 1;

                let mut value = next[i] - 1;

                for j in (0..i).rev() {
                    next[j] = value;

                    if value > 0 {
                        value -= 1;
                    }
                }
            }

            break;
        }
    }

    (next, next_keta)
}
