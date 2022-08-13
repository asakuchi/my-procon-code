use proconio::input;

fn main() {
    input! {
        h_1: usize,
        w_1: usize,
        a: [[usize; w_1]; h_1],
        h_2: usize,
        w_2: usize,
        b: [[usize; w_2]; h_2],
    }

    let diff_h = h_1 - h_2;
    let diff_w = w_1 - w_2;

    for mask_h in 0..1 << 10 {
        let mut count = 0;

        for i in 0..10 {
            if mask_h & 1 << i == 0 {
                count += 1;
            }
        }

        if count != diff_h {
            continue;
        }

        for mask_w in 0..1 << 10 {
            let mut count = 0;

            for i in 0..10 {
                if mask_w & 1 << i == 0 {
                    count += 1;
                }
            }

            if count != diff_w {
                continue;
            }

            // 一致するか

            let mut new_a = Vec::new();

            'create_loop: for i in 0..h_1 {
                let mut line = Vec::new();

                if mask_h & 1 << i == 0 {
                    continue 'create_loop;
                }

                for j in 0..w_1 {
                    if mask_w & 1 << j > 0 {
                        line.push(a[i][j]);
                    }
                }

                new_a.push(line);
            }

            if b == new_a {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
