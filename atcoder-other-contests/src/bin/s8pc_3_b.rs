use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }

    // 0行目に番兵を入れる
    let mut table = vec![vec![0; w]; h + 1];

    for i in 0..h {
        for j in 0..w {
            table[i + 1][j] = c[i][j].to_string().parse().unwrap();
        }
    }

    // 番兵込み
    let h = h + 1;

    let mut result = 0;

    for i in 1..h {
        for j in 0..w {
            let mut table = table.clone();

            // (i, j) を消す
            table[i][j] = 0;
            compress(h, j, &mut table);

            let score = chain(h, w, k, &mut table, 0);

            result = result.max(score);
        }
    }

    println!("{}", result);
}

fn chain(h: usize, w: usize, k: usize, table: &mut Vec<Vec<usize>>, chain_count: u32) -> usize {
    let mut sum = 0;

    // k同じものを消滅
    for i in 0..h {
        sum += remove(i, w, k, table);
    }

    if sum == 0 {
        return 0;
    }

    let mut score = 2usize.pow(chain_count) * sum;

    // 空きを埋める
    for j in 0..w {
        compress(h, j, table);
    }

    // 繰り返し
    if sum != 0 {
        score += chain(h, w, k, table, chain_count + 1);
    }

    score
}

fn remove(i: usize, w: usize, k: usize, table: &mut Vec<Vec<usize>>) -> usize {
    // 消滅で消えた数字の数の和
    let mut sum = 0;

    for j in 0..w {
        if table[i][j] == 0 {
            continue;
        }

        let mut same = 1;
        let mut same_index = j;

        for l in j + 1..w {
            if table[i][l] == 0 {
                break;
            }

            if table[i][j] == table[i][l] {
                same += 1;
                same_index = l;
            } else {
                break;
            }
        }

        if same >= k {
            for l in j..=same_index {
                sum += table[i][l];

                table[i][l] = 0;
            }
        }
    }

    sum
}

///
/// 列をつめる
///
fn compress(h: usize, j: usize, table: &mut Vec<Vec<usize>>) {
    let mut compress = Vec::new();

    // 上から詰める
    for i in 0..h {
        if table[i][j] != 0 {
            compress.push(table[i][j]);
        }
    }

    for i in (0..h).rev() {
        if let Some(value) = compress.pop() {
            table[i][j] = value;
        } else {
            table[i][j] = 0;
        }
    }
}
