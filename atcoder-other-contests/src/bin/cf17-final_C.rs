use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let mut count = vec![0; 13];

    for i in 0..n {
        count[d[i]] += 1;

        if count[0] >= 1 || count[d[i]] == 3 || count[12] > 1 {
            println!("0");
            return;
        }
    }

    // 探索頂点
    let mut vertex = vec![false; 24];
    // 選択肢があるもの
    let mut candi = Vec::new();

    // 高橋君
    vertex[0] = true;

    if count[12] == 1 {
        vertex[12] = true;
    }

    for i in 1..=11 {
        if count[i] == 1 {
            candi.push(i);
        } else if count[i] == 2 {
            vertex[i] = true;
            vertex[24 - i] = true;
        }
    }

    let mut result = 0;

    for mask in 0..1 << candi.len() {
        for i in 0..candi.len() {
            if mask & 1 << i > 0 {
                vertex[candi[i]] = true;
                vertex[24 - candi[i]] = false;
            } else {
                vertex[candi[i]] = false;
                vertex[24 - candi[i]] = true;
            }
        }

        let mut score = 1_000_000_000;

        for i in 0..24 {
            for j in i + 1..24 {
                if vertex[i] & vertex[j] {
                    let d1 = (i as isize - j as isize).abs();
                    let d2 = 24 - d1;
                    let d = d1.min(d2);

                    score = score.min(d);
                }
            }
        }

        result = result.max(score);
    }

    println!("{}", result);
}
