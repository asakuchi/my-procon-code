use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        matrix: [[usize; c]; r],
    }

    let mut result = 0;

    for mask in 0..1 << r {
        let mut total_good_count = 0;

        for j in 0..c {
            let mut line_good_count = 0;

            for i in 0..r {
                let mut is_good = matrix[i][j] == 0;

                if mask & 1 << i > 0 {
                    is_good = !is_good;
                }

                if is_good {
                    line_good_count += 1;
                }
            }

            if line_good_count < (r + 1) / 2 {
                // 列をひっくり返す
                line_good_count = r - line_good_count;
            }

            total_good_count += line_good_count;
        }

        result = result.max(total_good_count);
    }

    println!("{}", result);
}
