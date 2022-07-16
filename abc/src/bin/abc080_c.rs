use proconio::input;

fn main() {
    input! {
        n: usize,
        f: [[usize; 10]; n],
        p: [[isize; 11]; n],
    }

    let mut result = -1_000_000_000_000;

    for mask in 1..1 << 10 {
        let mut score = 0;

        for i in 0..n {
            let mut open_count = 0;

            for j in 0..10 {
                if f[i][j] == 1 && mask & 1 << j > 0 {
                    open_count += 1;
                }
            }

            score += p[i][open_count];
        }

        result = result.max(score);
    }

    println!("{}", result);
}
